
use database::{native_enums::*,models::*};
use utoipa::{
    openapi::{
        self,
        security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
         SchemaType,
    },
    Modify, OpenApi, ToSchema,
};
use crate::{handlers::*, serialization::*};
use chrono::{SecondsFormat, Utc};

#[derive(OpenApi)]
#[openapi(
  info(
    version = "1.1.0",
    license (name = "MIT", url = ""),
    description = "🦀 prediction event api documentation",
    contact (name = "goni098", url = "https://github.com/goni098")
  ),
  paths(
        get_events,
        get_event,
        get_user_statistics,
        sign_in  
    ),
    components(
      schemas(
        // query

        // body
        SignInPayload,

        // responses
        PredictionEventWithTickets,
        PaginatedEventsWithTickets,
        PredictionEvent,
        Ticket,
        Token,
        UserStatistics,

        // custom types
        DateTimeWithTimeZone,
        Decimal,

        // enums
        Side,
        Rst,

      ),
      responses()
    ),
    modifiers(&BearerSecurity)
  )]
pub struct ApiDoc;

struct BearerSecurity;

impl Modify for BearerSecurity {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "BearerAuth",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }
    }
}

struct DateTimeWithTimeZone;
struct Decimal;

impl<'__s> ToSchema<'__s> for DateTimeWithTimeZone {
    fn schema() -> (&'__s str, openapi::RefOr<openapi::schema::Schema>) {
        (
            "DateTimeWithTimeZone",
            openapi::ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("RFC 3339, ISO 8601 date and time string"))
                .default(Some(
                    Utc::now()
                        .to_rfc3339_opts(SecondsFormat::Millis, true)
                        .into(),
                ))
                .into(),
        )
    }
}

impl<'__s> ToSchema<'__s> for Decimal {
    fn schema() -> (&'__s str, openapi::RefOr<openapi::schema::Schema>) {
        (
            "Decimal",
            openapi::ObjectBuilder::new()
                .schema_type(SchemaType::String)
                .description(Some("Decimal value"))
                .default(Some(serde_json::Value::String("9000.00".to_string())))
                .into(),
        )
    }
}