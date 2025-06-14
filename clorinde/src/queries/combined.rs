// This file was generated with `clorinde`. Do not modify.

#[derive(Clone, Copy, Debug)]
pub struct GetNextDepsNearPointParams {
    pub lat: f64,
    pub lon: f64,
    pub num_stops: i64,
    pub time: i32,
}
#[derive(Debug, Clone, PartialEq)]
pub struct DepartureResult {
    pub agency: String,
    pub sortabletime: i32,
    pub stop_id: String,
    pub stop_code: Option<String>,
    pub stop_name: String,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub route_id: String,
    pub route_short_name: Option<String>,
    pub route_long_name: Option<String>,
    pub route_color: Option<String>,
    pub route_text_color: Option<String>,
    pub route_type: Option<i32>,
    pub trip_headsign: Option<String>,
    pub direction_id: Option<i32>,
}
pub struct DepartureResultBorrowed<'a> {
    pub agency: &'a str,
    pub sortabletime: i32,
    pub stop_id: &'a str,
    pub stop_code: Option<&'a str>,
    pub stop_name: &'a str,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub route_id: &'a str,
    pub route_short_name: Option<&'a str>,
    pub route_long_name: Option<&'a str>,
    pub route_color: Option<&'a str>,
    pub route_text_color: Option<&'a str>,
    pub route_type: Option<i32>,
    pub trip_headsign: Option<&'a str>,
    pub direction_id: Option<i32>,
}
impl<'a> From<DepartureResultBorrowed<'a>> for DepartureResult {
    fn from(
        DepartureResultBorrowed {
            agency,
            sortabletime,
            stop_id,
            stop_code,
            stop_name,
            stop_lat,
            stop_lon,
            route_id,
            route_short_name,
            route_long_name,
            route_color,
            route_text_color,
            route_type,
            trip_headsign,
            direction_id,
        }: DepartureResultBorrowed<'a>,
    ) -> Self {
        Self {
            agency: agency.into(),
            sortabletime,
            stop_id: stop_id.into(),
            stop_code: stop_code.map(|v| v.into()),
            stop_name: stop_name.into(),
            stop_lat,
            stop_lon,
            route_id: route_id.into(),
            route_short_name: route_short_name.map(|v| v.into()),
            route_long_name: route_long_name.map(|v| v.into()),
            route_color: route_color.map(|v| v.into()),
            route_text_color: route_text_color.map(|v| v.into()),
            route_type,
            trip_headsign: trip_headsign.map(|v| v.into()),
            direction_id,
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct DepartureResultQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<DepartureResultBorrowed, tokio_postgres::Error>,
    mapper: fn(DepartureResultBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> DepartureResultQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(
        self,
        mapper: fn(DepartureResultBorrowed) -> R,
    ) -> DepartureResultQuery<'c, 'a, 's, C, R, N> {
        DepartureResultQuery {
            client: self.client,
            params: self.params,
            stmt: self.stmt,
            extractor: self.extractor,
            mapper,
        }
    }
    pub async fn one(self) -> Result<T, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        let row = self.client.query_one(stmt, &self.params).await?;
        Ok((self.mapper)((self.extractor)(&row)?))
    }
    pub async fn all(self) -> Result<Vec<T>, tokio_postgres::Error> {
        self.iter().await?.try_collect().await
    }
    pub async fn opt(self) -> Result<Option<T>, tokio_postgres::Error> {
        let stmt = self.stmt.prepare(self.client).await?;
        Ok(self
            .client
            .query_opt(stmt, &self.params)
            .await?
            .map(|row| {
                let extracted = (self.extractor)(&row)?;
                Ok((self.mapper)(extracted))
            })
            .transpose()?)
    }
    pub async fn iter(
        self,
    ) -> Result<
        impl futures::Stream<Item = Result<T, tokio_postgres::Error>> + 'c,
        tokio_postgres::Error,
    > {
        let stmt = self.stmt.prepare(self.client).await?;
        let it = self
            .client
            .query_raw(stmt, crate::slice_iter(&self.params))
            .await?
            .map(move |res| {
                res.and_then(|row| {
                    let extracted = (self.extractor)(&row)?;
                    Ok((self.mapper)(extracted))
                })
            })
            .into_stream();
        Ok(it)
    }
}
pub fn get_next_deps_near_point() -> GetNextDepsNearPointStmt {
    GetNextDepsNearPointStmt(crate::client::async_::Stmt::new(
        "WITH next_deps_of_nearest_stops AS ( WITH n_nearest_stops AS ( SELECT * FROM Stops ORDER BY stop_lat_lon <-> point ($1, $2) LIMIT $3 ) SELECT *, ROW_NUMBER() OVER (PARTITION BY (agency, route_id, direction_id) ORDER BY SortableTime ASC) nth_of_route FROM StopTimes JOIN n_nearest_stops USING (agency, stop_id) JOIN trips USING (agency, trip_id) WHERE SortableTime >= $4 AND SortableTime < $4 + 7200 ) SELECT agency, sortabletime, stop_id, stop_code, stop_name, stop_lat_lon[0] as stop_lat, stop_lat_lon[1] as stop_lon, route_id, route_short_name, route_long_name, route_color, route_text_color, route_type, trip_headsign, direction_id FROM next_deps_of_nearest_stops JOIN routes USING (agency, route_id) WHERE nth_of_route = 1",
    ))
}
pub struct GetNextDepsNearPointStmt(crate::client::async_::Stmt);
impl GetNextDepsNearPointStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        lat: &'a f64,
        lon: &'a f64,
        num_stops: &'a i64,
        time: &'a i32,
    ) -> DepartureResultQuery<'c, 'a, 's, C, DepartureResult, 4> {
        DepartureResultQuery {
            client,
            params: [lat, lon, num_stops, time],
            stmt: &mut self.0,
            extractor: |
                row: &tokio_postgres::Row,
            | -> Result<DepartureResultBorrowed, tokio_postgres::Error> {
                Ok(DepartureResultBorrowed {
                    agency: row.try_get(0)?,
                    sortabletime: row.try_get(1)?,
                    stop_id: row.try_get(2)?,
                    stop_code: row.try_get(3)?,
                    stop_name: row.try_get(4)?,
                    stop_lat: row.try_get(5)?,
                    stop_lon: row.try_get(6)?,
                    route_id: row.try_get(7)?,
                    route_short_name: row.try_get(8)?,
                    route_long_name: row.try_get(9)?,
                    route_color: row.try_get(10)?,
                    route_text_color: row.try_get(11)?,
                    route_type: row.try_get(12)?,
                    trip_headsign: row.try_get(13)?,
                    direction_id: row.try_get(14)?,
                })
            },
            mapper: |it| DepartureResult::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetNextDepsNearPointParams,
        DepartureResultQuery<'c, 'a, 's, C, DepartureResult, 4>,
        C,
    > for GetNextDepsNearPointStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a GetNextDepsNearPointParams,
    ) -> DepartureResultQuery<'c, 'a, 's, C, DepartureResult, 4> {
        self.bind(
            client,
            &params.lat,
            &params.lon,
            &params.num_stops,
            &params.time,
        )
    }
}
