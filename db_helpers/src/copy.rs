/**
 * Utilities for using postgres' COPY (along with tokio-postgres copy_in)
 */

use std::convert::From;
use std::pin::Pin;
use tokio_postgres::{self, binary_copy::BinaryCopyInWriter, types::{ToSql, Type}};
use clorinde::deadpool_postgres::Transaction;
use anyhow::Result;
use futures::pin_mut;

pub trait BinaryCopy {
    /// Returns a static list of database column types, as defined by tokio_postgres.
    fn get_col_types() -> &'static [Type];
    /// Returns the SQL COPY command used to copy from stdin.
    fn get_copy_command() -> &'static str;
    /// Write the data type as a row to `writer`.
    #[allow(async_fn_in_trait)]
    async fn write_row(&self, writer: Pin<&mut BinaryCopyInWriter>) -> Result<(), tokio_postgres::Error>;
}

impl BinaryCopy for gtfs::StopTime {
    fn get_col_types() -> &'static [Type] {
        &[
            Type::TEXT, // agency
            Type::INT4, // SortableTime
            Type::TEXT, // trip_id
            Type::INT4, // arrival_time
            Type::INT4, // departure_time
            Type::TEXT, // stop_id
            Type::TEXT, // location_group_id
            Type::TEXT, // location_id
            Type::INT4, // stop_sequence
            Type::TEXT, // stop_headsign
            Type::INT4, // start_pickup_drop_off_window
            Type::INT4, // end_pickup_drop_off_window
            Type::INT4, // pickup_type
            Type::INT4, // drop_off_type
            Type::INT4, // continuous_pickup
            Type::INT4, // continuous_drop_off
            Type::FLOAT4, // shape_dist_traveled
            Type::INT4, // timepoint
            Type::TEXT, // pickup_booking_rule_id
            Type::TEXT, // drop_off_booking_rule_id
        ]
    }

    fn get_copy_command() -> &'static str {
        "COPY StopTimes FROM STDIN BINARY"
    }

    async fn write_row(&self, writer: Pin<&mut BinaryCopyInWriter>) -> Result<(), tokio_postgres::Error> {
        let row: &[&'_ (dyn ToSql + Sync)] = &[
            &self.agency,
            &i32::from(
                self.departure_time
                    .or(self.end_pickup_drop_off_window)
                    .or(self.arrival_time)
                    .or(self.start_pickup_drop_off_window)
                    .expect("Stop time did not have any associated times (this doesn't meet GTFS spec)")
            ),
            &self.trip_id,
            &self.arrival_time.map(|t| i32::from(t)),
            &self.departure_time.map(|t| i32::from(t)),
            &self.stop_id,
            &self.location_group_id,
            &self.location_id,
            &self.stop_sequence,
            &self.stop_headsign,
            &self.start_pickup_drop_off_window.map(|t| i32::from(t)),
            &self.end_pickup_drop_off_window.map(|t| i32::from(t)),
            &self.pickup_type,
            &self.drop_off_type,
            &self.continuous_pickup,
            &self.continuous_drop_off,
            &self.shape_dist_traveled,
            &self.timepoint,
            &self.pickup_booking_rule_id,
            &self.drop_off_booking_rule_id,
        ];

        writer.write(row).await?;

        Ok(())
    }
}

impl BinaryCopy for gtfs::Stop {
    fn get_col_types() -> &'static [Type] {
        &[
            Type::TEXT, // agency
            Type::TEXT, // stop_id
            Type::TEXT, // stop_code
            Type::TEXT, // stop_name
            Type::TEXT, // tts_stop_name
            Type::TEXT, // stop_desc
            Type::POINT, // stop_lat_lon
            Type::TEXT, // zone_id
            Type::TEXT, // stop_url
            Type::TEXT, // location_type
            Type::TEXT, // parent_station
            Type::TEXT, // stop_timezone
            Type::INT4, // wheelchair_boarding
            Type::TEXT, // level_id
            Type::TEXT, // platform_code
        ]
    }

    fn get_copy_command() -> &'static str {
        "COPY Stops FROM STDIN BINARY"
    }

    async fn write_row(&self, writer: Pin<&mut BinaryCopyInWriter>) -> Result<(), tokio_postgres::Error> {
        let row: &[&'_ (dyn ToSql + Sync)] = &[
            &self.agency,
            &self.stop_id,
            &self.stop_code,
            &self.stop_name,
            &self.tts_stop_name,
            &self.stop_desc,
            &geo_types::Point::new(self.stop_lat, self.stop_lon),
            &self.zone_id,
            &self.stop_url,
            &self.location_type,
            &self.parent_station,
            &self.stop_timezone,
            &self.wheelchair_boarding,
            &self.level_id,
            &self.platform_code,
        ];

        writer.write(row).await?;

        Ok(())
    }
}

impl BinaryCopy for gtfs::Trip {
    fn get_col_types() -> &'static [Type] {
        &[
            Type::TEXT, // agency
            Type::TEXT, // route_id
            Type::TEXT, // service_id
            Type::TEXT, // trip_id
            Type::TEXT, // trip_headsign
            Type::TEXT, // trip_short_name
            Type::INT4, // direction_id
            Type::TEXT, // block_id
            Type::TEXT, // shape_id
            Type::INT4, // wheelchair_accessible
            Type::INT4, // bikes_allowed
        ]
    }

    fn get_copy_command() -> &'static str {
        "COPY Trips FROM STDIN BINARY"
    }

    async fn write_row(&self, writer: Pin<&mut BinaryCopyInWriter>) -> Result<(), tokio_postgres::Error> {
        let row: &[&'_ (dyn ToSql + Sync)] = &[
            &self.agency,
            &self.route_id,
            &self.service_id,
            &self.trip_id,
            &self.trip_headsign,
            &self.trip_short_name,
            &self.direction_id,
            &self.block_id,
            &self.shape_id,
            &self.wheelchair_accessible,
            &self.bikes_allowed,
        ];

        writer.write(row).await?;

        Ok(())
    }
}

impl BinaryCopy for gtfs::Route {
    fn get_col_types() -> &'static [Type] {
        &[
            Type::TEXT, // agency
            Type::TEXT, // route_id
            Type::TEXT, // agency_id
            Type::TEXT, // route_short_name
            Type::TEXT, // route_long_name
            Type::TEXT, // route_desc
            Type::INT4, // route_type
            Type::TEXT, // route_url
            Type::TEXT, // route_color
            Type::TEXT, // route_text_color
            Type::INT4, // route_sort_order
            Type::INT4, // continuous_pickup
            Type::INT4, // continuous_drop_off
            Type::TEXT, // network_id
        ]
    }

    fn get_copy_command() -> &'static str {
        "COPY Routes FROM STDIN BINARY"
    }

    async fn write_row(&self, writer: Pin<&mut BinaryCopyInWriter>) -> Result<(), tokio_postgres::Error> {
        let row: &[&'_ (dyn ToSql + Sync)] = &[
            &self.agency,
            &self.route_id,
            &self.agency_id,
            &self.route_short_name,
            &self.route_long_name,
            &self.route_desc,
            &self.route_type,
            &self.route_url,
            &self.route_color,
            &self.route_text_color,
            &self.route_sort_order,
            &self.continuous_pickup,
            &self.continuous_drop_off,
            &self.network_id,
        ];

        writer.write(row).await?;

        Ok(())
    }
}

impl BinaryCopy for gtfs::Calendar {
    fn get_col_types() -> &'static [Type] {
        &[
            Type::TEXT, // agency
            Type::TEXT, // service_id
            Type::BOOL, // monday
            Type::BOOL, // tuesday
            Type::BOOL, // wednesday
            Type::BOOL, // thursday
            Type::BOOL, // friday
            Type::BOOL, // saturday
            Type::BOOL, // sunday
            Type::DATE, // start_date
            Type::DATE, // end_date
        ]
    }

    fn get_copy_command() -> &'static str {
        "COPY Calendar FROM STDIN BINARY"
    }

    async fn write_row(&self, writer: Pin<&mut BinaryCopyInWriter>) -> Result<(), tokio_postgres::Error> {
        let row: &[&'_ (dyn ToSql + Sync)] = &[
            &self.agency,
            &self.service_id,
            &self.monday,
            &self.tuesday,
            &self.wednesday,
            &self.thursday,
            &self.friday,
            &self.saturday,
            &self.sunday,
            &self.start_date,
            &self.end_date,
        ];

        writer.write(row).await?;

        Ok(())
    }
}

impl BinaryCopy for gtfs::CalendarDate {
    fn get_col_types() -> &'static [Type] {
        &[
            Type::TEXT, // agency
            Type::TEXT, // service_id
            Type::DATE, // date
            Type::INT4, // exception_type
        ]
    }

    fn get_copy_command() -> &'static str {
        "COPY CalendarDates FROM STDIN BINARY"
    }

    async fn write_row(&self, writer: Pin<&mut BinaryCopyInWriter>) -> Result<(), tokio_postgres::Error> {
        let row: &[&'_ (dyn ToSql + Sync)] = &[
            &self.agency,
            &self.service_id,
            &self.date,
            &self.exception_type,
        ];

        writer.write(row).await?;

        Ok(())
    }
}

pub async fn write_to_table<T: BinaryCopy>(it: impl Iterator<Item = T>, transaction: &Transaction<'_>) -> Result<u64> {
    let copy_in_sink = transaction.copy_in(T::get_copy_command()).await?;
    let col_types = T::get_col_types();
    let writer = BinaryCopyInWriter::new(copy_in_sink, col_types);
    pin_mut!(writer);
    for row in it {
        row.write_row(writer.as_mut()).await?;
    }
    let num_rows = writer.finish().await?;
    Ok(num_rows)
}