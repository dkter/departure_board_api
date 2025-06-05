// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertTripParams<
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
    pub route_id: T2,
    pub service_id: T3,
    pub trip_id: T4,
    pub trip_headsign: Option<T5>,
    pub trip_short_name: Option<T6>,
    pub direction_id: Option<i32>,
    pub block_id: Option<T7>,
    pub shape_id: Option<T8>,
    pub wheelchair_accessible: Option<i32>,
    pub bikes_allowed: Option<i32>,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub fn insert_trip() -> InsertTripStmt {
    InsertTripStmt(crate::client::async_::Stmt::new(
        "INSERT INTO Trips ( Agency, route_id, service_id, trip_id, trip_headsign, trip_short_name, direction_id, block_id, shape_id, wheelchair_accessible, bikes_allowed ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11 )",
    ))
}
pub struct InsertTripStmt(crate::client::async_::Stmt);
impl InsertTripStmt {
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
        route_id: &'a T2,
        service_id: &'a T3,
        trip_id: &'a T4,
        trip_headsign: &'a Option<T5>,
        trip_short_name: &'a Option<T6>,
        direction_id: &'a Option<i32>,
        block_id: &'a Option<T7>,
        shape_id: &'a Option<T8>,
        wheelchair_accessible: &'a Option<i32>,
        bikes_allowed: &'a Option<i32>,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[
                agency,
                route_id,
                service_id,
                trip_id,
                trip_headsign,
                trip_short_name,
                direction_id,
                block_id,
                shape_id,
                wheelchair_accessible,
                bikes_allowed,
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
        InsertTripParams<T1, T2, T3, T4, T5, T6, T7, T8>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertTripStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertTripParams<T1, T2, T3, T4, T5, T6, T7, T8>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.agency,
            &params.route_id,
            &params.service_id,
            &params.trip_id,
            &params.trip_headsign,
            &params.trip_short_name,
            &params.direction_id,
            &params.block_id,
            &params.shape_id,
            &params.wheelchair_accessible,
            &params.bikes_allowed,
        ))
    }
}
