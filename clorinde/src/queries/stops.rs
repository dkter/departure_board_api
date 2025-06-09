// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertStopParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
    T7: crate::StringSql,
    T8: crate::StringSql,
    T9: crate::StringSql,
    T10: crate::StringSql,
    T11: crate::StringSql,
    T12: crate::StringSql,
    T13: crate::StringSql,
> {
    pub agency: T1,
    pub stop_id: T2,
    pub stop_code: Option<T3>,
    pub stop_name: T4,
    pub tts_stop_name: Option<T5>,
    pub stop_desc: Option<T6>,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub zone_id: Option<T7>,
    pub stop_url: Option<T8>,
    pub location_type: Option<T9>,
    pub parent_station: Option<T10>,
    pub stop_timezone: Option<T11>,
    pub wheelchair_boarding: Option<i32>,
    pub level_id: Option<T12>,
    pub platform_code: Option<T13>,
}
#[derive(Clone, Copy, Debug)]
pub struct GetNNearestStopsParams {
    pub lat: f64,
    pub lon: f64,
    pub limit: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
    pub agency: String,
    pub stop_id: String,
    pub stop_code: Option<String>,
    pub stop_name: String,
    pub tts_stop_name: Option<String>,
    pub stop_desc: Option<String>,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub zone_id: Option<String>,
    pub stop_url: Option<String>,
    pub location_type: Option<String>,
    pub parent_station: Option<String>,
    pub stop_timezone: Option<String>,
    pub wheelchair_boarding: Option<i32>,
    pub level_id: Option<String>,
    pub platform_code: Option<String>,
}
pub struct StopBorrowed<'a> {
    pub agency: &'a str,
    pub stop_id: &'a str,
    pub stop_code: Option<&'a str>,
    pub stop_name: &'a str,
    pub tts_stop_name: Option<&'a str>,
    pub stop_desc: Option<&'a str>,
    pub stop_lat: f64,
    pub stop_lon: f64,
    pub zone_id: Option<&'a str>,
    pub stop_url: Option<&'a str>,
    pub location_type: Option<&'a str>,
    pub parent_station: Option<&'a str>,
    pub stop_timezone: Option<&'a str>,
    pub wheelchair_boarding: Option<i32>,
    pub level_id: Option<&'a str>,
    pub platform_code: Option<&'a str>,
}
impl<'a> From<StopBorrowed<'a>> for Stop {
    fn from(
        StopBorrowed {
            agency,
            stop_id,
            stop_code,
            stop_name,
            tts_stop_name,
            stop_desc,
            stop_lat,
            stop_lon,
            zone_id,
            stop_url,
            location_type,
            parent_station,
            stop_timezone,
            wheelchair_boarding,
            level_id,
            platform_code,
        }: StopBorrowed<'a>,
    ) -> Self {
        Self {
            agency: agency.into(),
            stop_id: stop_id.into(),
            stop_code: stop_code.map(|v| v.into()),
            stop_name: stop_name.into(),
            tts_stop_name: tts_stop_name.map(|v| v.into()),
            stop_desc: stop_desc.map(|v| v.into()),
            stop_lat,
            stop_lon,
            zone_id: zone_id.map(|v| v.into()),
            stop_url: stop_url.map(|v| v.into()),
            location_type: location_type.map(|v| v.into()),
            parent_station: parent_station.map(|v| v.into()),
            stop_timezone: stop_timezone.map(|v| v.into()),
            wheelchair_boarding,
            level_id: level_id.map(|v| v.into()),
            platform_code: platform_code.map(|v| v.into()),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct StopQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<StopBorrowed, tokio_postgres::Error>,
    mapper: fn(StopBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StopQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(StopBorrowed) -> R) -> StopQuery<'c, 'a, 's, C, R, N> {
        StopQuery {
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
pub fn insert_stop() -> InsertStopStmt {
    InsertStopStmt(crate::client::async_::Stmt::new(
        "INSERT INTO Stops ( Agency, stop_id, stop_code, stop_name, tts_stop_name, stop_desc, stop_lat_lon, zone_id, stop_url, location_type, parent_station, stop_timezone, wheelchair_boarding, level_id, platform_code ) VALUES ( $1, $2, $3, $4, $5, $6, (point ($7, $8)), $9, $10, $11, $12, $13, $14, $15, $16 )",
    ))
}
pub struct InsertStopStmt(crate::client::async_::Stmt);
impl InsertStopStmt {
    pub async fn bind<
        'c,
        'a,
        's,
        C: GenericClient,
        T1: crate::StringSql,
        T2: crate::StringSql,
        T3: crate::StringSql,
        T4: crate::StringSql,
        T5: crate::StringSql,
        T6: crate::StringSql,
        T7: crate::StringSql,
        T8: crate::StringSql,
        T9: crate::StringSql,
        T10: crate::StringSql,
        T11: crate::StringSql,
        T12: crate::StringSql,
        T13: crate::StringSql,
    >(
        &'s mut self,
        client: &'c C,
        agency: &'a T1,
        stop_id: &'a T2,
        stop_code: &'a Option<T3>,
        stop_name: &'a T4,
        tts_stop_name: &'a Option<T5>,
        stop_desc: &'a Option<T6>,
        stop_lat: &'a f64,
        stop_lon: &'a f64,
        zone_id: &'a Option<T7>,
        stop_url: &'a Option<T8>,
        location_type: &'a Option<T9>,
        parent_station: &'a Option<T10>,
        stop_timezone: &'a Option<T11>,
        wheelchair_boarding: &'a Option<i32>,
        level_id: &'a Option<T12>,
        platform_code: &'a Option<T13>,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[
                agency,
                stop_id,
                stop_code,
                stop_name,
                tts_stop_name,
                stop_desc,
                stop_lat,
                stop_lon,
                zone_id,
                stop_url,
                location_type,
                parent_station,
                stop_timezone,
                wheelchair_boarding,
                level_id,
                platform_code,
            ])
            .await
    }
}
impl<
    'a,
    C: GenericClient + Send + Sync,
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
    T7: crate::StringSql,
    T8: crate::StringSql,
    T9: crate::StringSql,
    T10: crate::StringSql,
    T11: crate::StringSql,
    T12: crate::StringSql,
    T13: crate::StringSql,
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        InsertStopParams<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertStopStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertStopParams<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.agency,
            &params.stop_id,
            &params.stop_code,
            &params.stop_name,
            &params.tts_stop_name,
            &params.stop_desc,
            &params.stop_lat,
            &params.stop_lon,
            &params.zone_id,
            &params.stop_url,
            &params.location_type,
            &params.parent_station,
            &params.stop_timezone,
            &params.wheelchair_boarding,
            &params.level_id,
            &params.platform_code,
        ))
    }
}
pub fn get_n_nearest_stops() -> GetNNearestStopsStmt {
    GetNNearestStopsStmt(crate::client::async_::Stmt::new(
        "SELECT Agency, stop_id, stop_code, stop_name, tts_stop_name, stop_desc, stop_lat_lon[0] AS stop_lat, stop_lat_lon[1] AS stop_lon, zone_id, stop_url, location_type, parent_station, stop_timezone, wheelchair_boarding, level_id, platform_code FROM Stops ORDER BY stop_lat_lon <-> point ($1, $2) LIMIT $3",
    ))
}
pub struct GetNNearestStopsStmt(crate::client::async_::Stmt);
impl GetNNearestStopsStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
        lat: &'a f64,
        lon: &'a f64,
        limit: &'a i64,
    ) -> StopQuery<'c, 'a, 's, C, Stop, 3> {
        StopQuery {
            client,
            params: [lat, lon, limit],
            stmt: &mut self.0,
            extractor: |row: &tokio_postgres::Row| -> Result<StopBorrowed, tokio_postgres::Error> {
                Ok(StopBorrowed {
                    agency: row.try_get(0)?,
                    stop_id: row.try_get(1)?,
                    stop_code: row.try_get(2)?,
                    stop_name: row.try_get(3)?,
                    tts_stop_name: row.try_get(4)?,
                    stop_desc: row.try_get(5)?,
                    stop_lat: row.try_get(6)?,
                    stop_lon: row.try_get(7)?,
                    zone_id: row.try_get(8)?,
                    stop_url: row.try_get(9)?,
                    location_type: row.try_get(10)?,
                    parent_station: row.try_get(11)?,
                    stop_timezone: row.try_get(12)?,
                    wheelchair_boarding: row.try_get(13)?,
                    level_id: row.try_get(14)?,
                    platform_code: row.try_get(15)?,
                })
            },
            mapper: |it| Stop::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetNNearestStopsParams,
        StopQuery<'c, 'a, 's, C, Stop, 3>,
        C,
    > for GetNNearestStopsStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a GetNNearestStopsParams,
    ) -> StopQuery<'c, 'a, 's, C, Stop, 3> {
        self.bind(client, &params.lat, &params.lon, &params.limit)
    }
}
