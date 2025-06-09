// This file was generated with `clorinde`. Do not modify.

#[derive(Debug)]
pub struct InsertRouteParams<
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
> {
    pub agency: T1,
    pub route_id: T2,
    pub agency_id: Option<T3>,
    pub route_short_name: Option<T4>,
    pub route_long_name: Option<T5>,
    pub route_desc: Option<T6>,
    pub route_type: Option<i32>,
    pub route_url: Option<T7>,
    pub route_color: Option<T8>,
    pub route_text_color: Option<T9>,
    pub route_sort_order: Option<i32>,
    pub continuous_pickup: Option<i32>,
    pub continuous_drop_off: Option<i32>,
    pub network_id: Option<T10>,
}
use crate::client::async_::GenericClient;
use futures::{self, StreamExt, TryStreamExt};
pub fn insert_route() -> InsertRouteStmt {
    InsertRouteStmt(crate::client::async_::Stmt::new(
        "INSERT INTO Routes ( Agency, route_id, agency_id, route_short_name, route_long_name, route_desc, route_type, route_url, route_color, route_text_color, route_sort_order, continuous_pickup, continuous_drop_off, network_id ) VALUES ( $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14 )",
    ))
}
pub struct InsertRouteStmt(crate::client::async_::Stmt);
impl InsertRouteStmt {
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
    >(
        &'s mut self,
        client: &'c C,
        agency: &'a T1,
        route_id: &'a T2,
        agency_id: &'a Option<T3>,
        route_short_name: &'a Option<T4>,
        route_long_name: &'a Option<T5>,
        route_desc: &'a Option<T6>,
        route_type: &'a Option<i32>,
        route_url: &'a Option<T7>,
        route_color: &'a Option<T8>,
        route_text_color: &'a Option<T9>,
        route_sort_order: &'a Option<i32>,
        continuous_pickup: &'a Option<i32>,
        continuous_drop_off: &'a Option<i32>,
        network_id: &'a Option<T10>,
    ) -> Result<u64, tokio_postgres::Error> {
        let stmt = self.0.prepare(client).await?;
        client
            .execute(stmt, &[
                agency,
                route_id,
                agency_id,
                route_short_name,
                route_long_name,
                route_desc,
                route_type,
                route_url,
                route_color,
                route_text_color,
                route_sort_order,
                continuous_pickup,
                continuous_drop_off,
                network_id,
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
>
    crate::client::async_::Params<
        'a,
        'a,
        'a,
        InsertRouteParams<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>,
        std::pin::Pin<
            Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
        >,
        C,
    > for InsertRouteStmt
{
    fn params(
        &'a mut self,
        client: &'a C,
        params: &'a InsertRouteParams<T1, T2, T3, T4, T5, T6, T7, T8, T9, T10>,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<u64, tokio_postgres::Error>> + Send + 'a>,
    > {
        Box::pin(self.bind(
            client,
            &params.agency,
            &params.route_id,
            &params.agency_id,
            &params.route_short_name,
            &params.route_long_name,
            &params.route_desc,
            &params.route_type,
            &params.route_url,
            &params.route_color,
            &params.route_text_color,
            &params.route_sort_order,
            &params.continuous_pickup,
            &params.continuous_drop_off,
            &params.network_id,
        ))
    }
}
