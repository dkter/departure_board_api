// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertStopTimeParams<
    T1: crate::StringSql,
    T2: crate::StringSql,
    T3: crate::StringSql,
    T4: crate::StringSql,
    T5: crate::StringSql,
    T6: crate::StringSql,
    T7: crate::StringSql,
    T8: crate::StringSql,
> {
    pub agency: T1,
    pub departure_time: Option<i32>,
    pub end_pickup_drop_off_window: Option<i32>,
    pub arrival_time: Option<i32>,
    pub start_pickup_drop_off_window: Option<i32>,
    pub trip_id: T2,
    pub stop_id: Option<T3>,
    pub location_group_id: Option<T4>,
    pub location_id: Option<T5>,
    pub stop_sequence: i32,
    pub stop_headsign: Option<T6>,
    pub pickup_type: Option<i32>,
    pub drop_off_type: Option<i32>,
    pub continuous_pickup: Option<i32>,
    pub continuous_drop_off: Option<i32>,
    pub shape_dist_traveled: Option<f32>,
    pub timepoint: Option<i32>,
    pub pickup_booking_rule_id: Option<T7>,
    pub drop_off_booking_rule_id: Option<T8>,
}
#[derive(Debug)]
pub struct GetNextDeparturesAfterTimeParams<T1: crate::StringSql> {
    pub time: i32,
    pub stop_id: T1,
    pub limit: i64,
}
#[derive(Debug, Clone, PartialEq)]
pub struct StopTimes {
    pub agency: String,
    pub sortabletime: i32,
    pub trip_id: String,
    pub arrival_time: Option<i32>,
    pub departure_time: Option<i32>,
    pub stop_id: Option<String>,
    pub location_group_id: Option<String>,
    pub location_id: Option<String>,
    pub stop_sequence: i32,
    pub stop_headsign: Option<String>,
    pub start_pickup_drop_off_window: Option<i32>,
    pub end_pickup_drop_off_window: Option<i32>,
    pub pickup_type: Option<i32>,
    pub drop_off_type: Option<i32>,
    pub continuous_pickup: Option<i32>,
    pub continuous_drop_off: Option<i32>,
    pub shape_dist_traveled: Option<f32>,
    pub timepoint: Option<i32>,
    pub pickup_booking_rule_id: Option<String>,
    pub drop_off_booking_rule_id: Option<String>,
}
pub struct StopTimesBorrowed<'a> {
    pub agency: &'a str,
    pub sortabletime: i32,
    pub trip_id: &'a str,
    pub arrival_time: Option<i32>,
    pub departure_time: Option<i32>,
    pub stop_id: Option<&'a str>,
    pub location_group_id: Option<&'a str>,
    pub location_id: Option<&'a str>,
    pub stop_sequence: i32,
    pub stop_headsign: Option<&'a str>,
    pub start_pickup_drop_off_window: Option<i32>,
    pub end_pickup_drop_off_window: Option<i32>,
    pub pickup_type: Option<i32>,
    pub drop_off_type: Option<i32>,
    pub continuous_pickup: Option<i32>,
    pub continuous_drop_off: Option<i32>,
    pub shape_dist_traveled: Option<f32>,
    pub timepoint: Option<i32>,
    pub pickup_booking_rule_id: Option<&'a str>,
    pub drop_off_booking_rule_id: Option<&'a str>,
}
impl<'a> From<StopTimesBorrowed<'a>> for StopTimes {
    fn from(
        StopTimesBorrowed {
            agency,
            sortabletime,
            trip_id,
            arrival_time,
            departure_time,
            stop_id,
            location_group_id,
            location_id,
            stop_sequence,
            stop_headsign,
            start_pickup_drop_off_window,
            end_pickup_drop_off_window,
            pickup_type,
            drop_off_type,
            continuous_pickup,
            continuous_drop_off,
            shape_dist_traveled,
            timepoint,
            pickup_booking_rule_id,
            drop_off_booking_rule_id,
        }: StopTimesBorrowed<'a>,
    ) -> Self {
        Self {
            agency: agency.into(),
            sortabletime,
            trip_id: trip_id.into(),
            arrival_time,
            departure_time,
            stop_id: stop_id.map(|v| v.into()),
            location_group_id: location_group_id.map(|v| v.into()),
            location_id: location_id.map(|v| v.into()),
            stop_sequence,
            stop_headsign: stop_headsign.map(|v| v.into()),
            start_pickup_drop_off_window,
            end_pickup_drop_off_window,
            pickup_type,
            drop_off_type,
            continuous_pickup,
            continuous_drop_off,
            shape_dist_traveled,
            timepoint,
            pickup_booking_rule_id: pickup_booking_rule_id.map(|v| v.into()),
            drop_off_booking_rule_id: drop_off_booking_rule_id.map(|v| v.into()),
        }
    }
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub struct StopTimesQuery<'c, 'a, 's, C: GenericClient, T, const N: usize> {
    client: &'c C,
    params: [&'a (dyn postgres_types::ToSql + Sync); N],
    stmt: &'s mut crate::client::async_::Stmt,
    extractor: fn(&tokio_postgres::Row) -> Result<StopTimesBorrowed, tokio_postgres::Error>,
    mapper: fn(StopTimesBorrowed) -> T,
}
impl<'c, 'a, 's, C, T: 'c, const N: usize> StopTimesQuery<'c, 'a, 's, C, T, N>
where
    C: GenericClient,
{
    pub fn map<R>(self, mapper: fn(StopTimesBorrowed) -> R) -> StopTimesQuery<'c, 'a, 's, C, R, N> {
        StopTimesQuery {
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
pub fn insert_stop_time() -> InsertStopTimeStmt {
    InsertStopTimeStmt(crate::client::async_::Stmt::new(
        "INSERT INTO StopTimes ( Agency, SortableTime, trip_id, arrival_time, departure_time, stop_id, location_group_id, location_id, stop_sequence, stop_headsign, start_pickup_drop_off_window, end_pickup_drop_off_window, pickup_type, drop_off_type, continuous_pickup, continuous_drop_off, shape_dist_traveled, timepoint, pickup_booking_rule_id, drop_off_booking_rule_id ) VALUES ( $1, COALESCE($2, $3, $4, $5, -1), $6, $4, $2, $7, $8, $9, $10, $11, $5, $3, $12, $13, $14, $15, $16, $17, $18, $19 )",
    ))
}
pub struct InsertStopTimeStmt(crate::client::async_::Stmt);
impl InsertStopTimeStmt {
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
    >(
        &'s mut self,
        client: &'c C,
        agency: &'a T1,
        departure_time: &'a Option<i32>,
        end_pickup_drop_off_window: &'a Option<i32>,
        arrival_time: &'a Option<i32>,
        start_pickup_drop_off_window: &'a Option<i32>,
        trip_id: &'a T2,
        stop_id: &'a Option<T3>,
        location_group_id: &'a Option<T4>,
        location_id: &'a Option<T5>,
        stop_sequence: &'a i32,
        stop_headsign: &'a Option<T6>,
        pickup_type: &'a Option<i32>,
        drop_off_type: &'a Option<i32>,
        continuous_pickup: &'a Option<i32>,
        continuous_drop_off: &'a Option<i32>,
        shape_dist_traveled: &'a Option<f32>,
        timepoint: &'a Option<i32>,
        pickup_booking_rule_id: &'a Option<T7>,
        drop_off_booking_rule_id: &'a Option<T8>,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(
                stmt,
                &[
                    agency,
                    departure_time,
                    end_pickup_drop_off_window,
                    arrival_time,
                    start_pickup_drop_off_window,
                    trip_id,
                    stop_id,
                    location_group_id,
                    location_id,
                    stop_sequence,
                    stop_headsign,
                    pickup_type,
                    drop_off_type,
                    continuous_pickup,
                    continuous_drop_off,
                    shape_dist_traveled,
                    timepoint,
                    pickup_booking_rule_id,
                    drop_off_booking_rule_id,
                ],
            )
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
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        InsertStopTimeParams<T1, T2, T3, T4, T5, T6, T7, T8>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertStopTimeStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertStopTimeParams<T1, T2, T3, T4, T5, T6, T7, T8>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.agency,
            &params.departure_time,
            &params.end_pickup_drop_off_window,
            &params.arrival_time,
            &params.start_pickup_drop_off_window,
            &params.trip_id,
            &params.stop_id,
            &params.location_group_id,
            &params.location_id,
            &params.stop_sequence,
            &params.stop_headsign,
            &params.pickup_type,
            &params.drop_off_type,
            &params.continuous_pickup,
            &params.continuous_drop_off,
            &params.shape_dist_traveled,
            &params.timepoint,
            &params.pickup_booking_rule_id,
            &params.drop_off_booking_rule_id,
        ))
    }
}
pub fn get_next_departures_after_time() -> GetNextDeparturesAfterTimeStmt {
    GetNextDeparturesAfterTimeStmt(crate::client::async_::Stmt::new(
        "SELECT * FROM StopTimes WHERE SortableTime > $1 AND stop_id = $2 ORDER BY SortableTime ASC LIMIT $3",
    ))
}
pub struct GetNextDeparturesAfterTimeStmt(crate::client::async_::Stmt);
impl GetNextDeparturesAfterTimeStmt {
    pub fn bind<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>(
        &'s mut self,
        client: &'c C,
        time: &'a i32,
        stop_id: &'a T1,
        limit: &'a i64,
    ) -> StopTimesQuery<'c, 'a, 's, C, StopTimes, 3> {
        StopTimesQuery {
            client,
            params: [time, stop_id, limit],
            stmt: &mut self.0,
            extractor:
                |row: &tokio_postgres::Row| -> Result<StopTimesBorrowed, tokio_postgres::Error> {
                    Ok(StopTimesBorrowed {
                        agency: row.try_get(0)?,
                        sortabletime: row.try_get(1)?,
                        trip_id: row.try_get(2)?,
                        arrival_time: row.try_get(3)?,
                        departure_time: row.try_get(4)?,
                        stop_id: row.try_get(5)?,
                        location_group_id: row.try_get(6)?,
                        location_id: row.try_get(7)?,
                        stop_sequence: row.try_get(8)?,
                        stop_headsign: row.try_get(9)?,
                        start_pickup_drop_off_window: row.try_get(10)?,
                        end_pickup_drop_off_window: row.try_get(11)?,
                        pickup_type: row.try_get(12)?,
                        drop_off_type: row.try_get(13)?,
                        continuous_pickup: row.try_get(14)?,
                        continuous_drop_off: row.try_get(15)?,
                        shape_dist_traveled: row.try_get(16)?,
                        timepoint: row.try_get(17)?,
                        pickup_booking_rule_id: row.try_get(18)?,
                        drop_off_booking_rule_id: row.try_get(19)?,
                    })
                },
            mapper: |it| StopTimes::from(it),
        }
    }
}
impl<'c, 'a, 's, C: GenericClient, T1: crate::StringSql>
    crate::client::async_::Params<
        'c,
        'a,
        's,
        GetNextDeparturesAfterTimeParams<T1>,
        StopTimesQuery<'c, 'a, 's, C, StopTimes, 3>,
        C,
    > for GetNextDeparturesAfterTimeStmt
{
    fn params(
        &'s mut self,
        client: &'c C,
        params: &'a GetNextDeparturesAfterTimeParams<T1>,
    ) -> StopTimesQuery<'c, 'a, 's, C, StopTimes, 3> {
        self.bind(client, &params.time, &params.stop_id, &params.limit)
    }
}
pub fn delete_index() -> DeleteIndexStmt {
    DeleteIndexStmt(crate::client::async_::Stmt::new(
        "DROP INDEX IF EXISTS SortableTimeIndex",
    ))
}
pub struct DeleteIndexStmt(crate::client::async_::Stmt);
impl DeleteIndexStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[]).await
    }
}
pub fn create_index() -> CreateIndexStmt {
    CreateIndexStmt(crate::client::async_::Stmt::new(
        "CREATE INDEX SortableTimeIndex ON StopTimes USING HASH (SortableTime)",
    ))
}
pub struct CreateIndexStmt(crate::client::async_::Stmt);
impl CreateIndexStmt {
    pub async fn bind<'c, 'a, 's, C: GenericClient>(
        &'s mut self,
        client: &'c C,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client.execute(stmt, &[]).await
    }
}
