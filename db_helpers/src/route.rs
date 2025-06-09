use clorinde::queries::routes::InsertRouteParams;

pub fn route_to_db_record(route: gtfs::Route) -> InsertRouteParams<String, String, String, String, String, String, String, String, String, String> {
    InsertRouteParams {
        agency: route.agency,
        route_id: route.route_id,
        agency_id: route.agency_id,
        route_short_name: route.route_short_name,
        route_long_name: route.route_long_name,
        route_desc: route.route_desc,
        route_type: route.route_type,
        route_url: route.route_url,
        route_color: route.route_color,
        route_text_color: route.route_text_color,
        route_sort_order: route.route_sort_order,
        continuous_pickup: route.continuous_pickup,
        continuous_drop_off: route.continuous_drop_off,
        network_id: route.network_id,
    }
}