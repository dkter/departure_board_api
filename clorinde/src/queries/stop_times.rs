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
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
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
            .execute(stmt, &[
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
