-- noinspection SqlNoDataSourceInspectionForFile
CREATE TYPE brand_kind AS ENUM ('INDUSTRIAL', 'BRASS_MODELS');
CREATE TYPE brand_status AS ENUM ('ACTIVE', 'OUT_OF_BUSINESS');
CREATE TYPE gauge AS ENUM ('BROAD', 'MEDIUM', 'MINIMUM', 'NARROW', 'STANDARD');
CREATE TYPE organization_entity_type AS ENUM (
    'CIVIL_LAW_PARTNERSHIP',
    'ENTREPRENEURIAL_COMPANY',
    'GLOBAL_PARTNERSHIP',
    'LIMITED_COMPANY',
    'LIMITED_PARTNERSHIP',
    'LIMITED_PARTNERSHIP_LIMITED_COMPANY',
    'OTHER',
    'PUBLIC_INSTITUTION',
    'PUBLIC_LIMITED_COMPANY',
    'REGISTERED_SOLE_TRADER',
    'SOLE_TRADER',
    'STATE_OWNED_ENTERPRISE'
    );
CREATE TYPE railway_status AS ENUM ('ACTIVE', 'INACTIVE');
CREATE TYPE scale_standard AS ENUM ('BRITISH', 'JAPANESE', 'NEM', 'NMRA');

CREATE TABLE public.brands
(
    brand_id                 varchar(50) NOT NULL,
    name                     varchar(50) NOT NULL,
    registered_company_name  varchar(100),
    organization_entity_type organization_entity_type,
    group_name               varchar(100),
    description_en           varchar(2500),
    description_it           varchar(2500),
    kind                     brand_kind  NOT NULL,
    status                   brand_status,
    contact_email            varchar(250),
    contact_website_url      varchar(100),
    contact_phone            varchar(20),
    address_street_address   varchar(255),
    address_extended_address varchar(255),
    address_city             varchar(50),
    address_region           varchar(50),
    address_postal_code      varchar(10),
    address_country          varchar(2),
    socials_facebook         varchar(100),
    socials_instagram        varchar(100),
    socials_linkedin         varchar(100),
    socials_twitter          varchar(100),
    socials_youtube          varchar(100),
    created_at               timestamptz NOT NULL,
    last_modified_at         timestamptz,
    version                  integer     NOT NULL DEFAULT 1,
    CONSTRAINT "PK_brands" PRIMARY KEY (brand_id)
);

CREATE TABLE public.railways
(
    railway_id               varchar(50) NOT NULL,
    name                     varchar(50) NOT NULL,
    abbreviation             varchar(10),
    registered_company_name  varchar(250),
    organization_entity_type organization_entity_type,
    description_en           varchar(2500),
    description_it           varchar(2500),
    country                  varchar(3)  NOT NULL,
    operating_since          date,
    operating_until          date,
    status                   railway_status,
    gauge_meters             numeric(5, 3),
    track_gauge              gauge,
    headquarters             varchar(250) array,
    total_length_mi          numeric(7, 1),
    total_length_km          numeric(7, 1),
    contact_email            varchar(255),
    contact_website_url      varchar(100),
    contact_phone            varchar(20),
    socials_facebook         varchar(100),
    socials_instagram        varchar(100),
    socials_linkedin         varchar(100),
    socials_twitter          varchar(100),
    socials_youtube          varchar(100),
    created_at               timestamptz NOT NULL,
    last_modified_at         timestamptz,
    version                  integer     NOT NULL DEFAULT 1,
    CONSTRAINT "PK_railways" PRIMARY KEY (railway_id)
);

CREATE TABLE public.scales
(
    scale_id          varchar(50)    NOT NULL,
    name              varchar(50)    NOT NULL,
    ratio             numeric(6, 2) NOT NULL,
    gauge_millimeters numeric(4, 1),
    gauge_inches      numeric(5, 2),
    track_gauge       gauge          NOT NULL,
    description_en    varchar(2500),
    description_it    varchar(2500),
    standards         scale_standard array,
    created_at        timestamptz    NOT NULL,
    last_modified_at  timestamptz,
    version           integer        NOT NULL DEFAULT 1,
    CONSTRAINT "PK_scales" PRIMARY KEY (scale_id)
);

CREATE TYPE rolling_stock_category AS ENUM (
    'LOCOMOTIVE',
    'FREIGHT_CAR',
    'PASSENGER_CAR',
    'ELECTRIC_MULTIPLE_UNIT',
    'RAILCAR'
    );
CREATE TYPE catalog_item_category AS ENUM (
    'LOCOMOTIVES',
    'TRAIN_SETS',
    'STARTER_SETS',
    'FREIGHT_CARS',
    'PASSENGER_CARS',
    'ELECTRIC_MULTIPLE_UNITS',
    'RAILCARS'
    );
CREATE TYPE feature_flag AS ENUM ('YES', 'NO', 'NOT_AVAILABLE');
CREATE TYPE power_method AS ENUM ('AC', 'DC', 'TRIX_EXPRESS');
CREATE TYPE availability_status AS ENUM (
    'ANNOUNCED',
    'AVAILABLE',
    'DISCONTINUED'
    );
CREATE TYPE control AS ENUM (
    'DCC',
    'DCC_READY',
    'DCC_SOUND',
    'NO_DCC'
    );
CREATE TYPE dcc_interface AS ENUM (
    'MTC_21',
    'NEM_651',
    'NEM_652',
    'NEM_654',
    'NEXT_18',
    'NEXT_18_S',
    'PLUX_16',
    'PLUX_22',
    'PLUX_8'
    );
CREATE TYPE locomotive_type AS ENUM (
    'DIESEL_LOCOMOTIVE',
    'ELECTRIC_LOCOMOTIVE',
    'STEAM_LOCOMOTIVE'
    );
CREATE TYPE passenger_car_type AS ENUM (
    'BAGGAGE_CAR',
    'COMBINE_CAR',
    'COMPARTMENT_COACH',
    'DINING_CAR',
    'DOUBLE_DECKER',
    'DRIVING_TRAILER',
    'LOUNGE',
    'OBSERVATION',
    'OPEN_COACH',
    'RAILWAY_POST_OFFICE',
    'SLEEPING_CAR'
    );
CREATE TYPE electric_multiple_unit_type AS ENUM (
    'DRIVING_CAR',
    'HIGH_SPEED_TRAIN',
    'MOTOR_CAR',
    'POWER_CAR',
    'TRAILER_CAR',
    'TRAIN_SET'
    );
CREATE TYPE railcar_type AS ENUM (
    'POWER_CAR',
    'TRAILER_CAR'
    );
CREATE TYPE freight_car_type AS ENUM (
    'AUTO_TRANSPORT_CARS',
    'BRAKE_WAGON',
    'CONTAINER_CARS',
    'COVERED_FREIGHT_CARS',
    'DEEP_WELL_FLAT_CARS',
    'DUMP_CARS',
    'GONDOLA',
    'HEAVY_GOODS_WAGONS',
    'HINGED_COVER_WAGONS',
    'HOPPER_WAGON',
    'REFRIGERATOR_CARS',
    'SILO_CONTAINER_CARS',
    'SLIDE_TARPAULIN_WAGON',
    'SLIDING_WALL_BOXCARS',
    'SPECIAL_TRANSPORT',
    'STAKE_WAGONS',
    'SWING_ROOF_WAGON',
    'TANK_CARS',
    'TELESCOPE_HOOD_WAGONS'
    );
CREATE TYPE socket_type AS ENUM (
    'NONE',
    'NEM_355',
    'NEM_356',
    'NEM_357',
    'NEM_359',
    'NEM_360',
    'NEM_362',
    'NEM_365'
    );
CREATE TYPE service_level AS ENUM (
    'FIRST_CLASS',
    'SECOND_CLASS',
    'THIRD_CLASS',
    'FIRST_AND_SECOND_CLASS',
    'FIRST_SECOND_AND_THIRD_CLASS',
    'SECOND_AND_THIRD_CLASS'
    );


CREATE TABLE public.catalog_items
(
    catalog_item_id     varchar(76)           NOT NULL,
    brand_id            varchar(50)           NOT NULL,
    item_number         varchar(25)           NOT NULL,
    scale_id            varchar(25)           NOT NULL,
    category            catalog_item_category NOT NULL,
    description_en      varchar(2500),
    description_it      varchar(2500),
    details_en          varchar(2500),
    details_it          varchar(2500),
    power_method        power_method          NOT NULL,
    delivery_date       varchar(10),
    availability_status availability_status,
    count               integer               NOT NULL DEFAULT 1,
    created_at          timestamptz           NOT NULL,
    last_modified_at    timestamptz,
    version             integer               NOT NULL DEFAULT 1,
    CONSTRAINT "PK_catalog_items" PRIMARY KEY (catalog_item_id),
    CONSTRAINT "FK_catalog_items_brands" FOREIGN KEY (brand_id)
        REFERENCES public.brands (brand_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_catalog_items_scales" FOREIGN KEY (scale_id)
        REFERENCES public.scales (scale_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE TABLE public.rolling_stocks
(
    rolling_stock_id            uuid                   NOT NULL,
    catalog_item_id             varchar(65)            NOT NULL,
    railway_id                  varchar(25)            NOT NULL,
    rolling_stock_category      rolling_stock_category NOT NULL,
    epoch                       varchar(10)            NOT NULL,
    livery                      varchar(50),
    length_over_buffers_mm      numeric(9, 2),
    length_over_buffers_in      numeric(9, 2),
    type_name                   varchar(25),
    road_number                 varchar(50),
    series                      varchar(50),
    depot                       varchar(100),
    dcc_interface               dcc_interface,
    control                     control,
    electric_multiple_unit_type electric_multiple_unit_type,
    freight_car_type            freight_car_type,
    locomotive_type             locomotive_type,
    passenger_car_type          passenger_car_type,
    railcar_type                railcar_type,
    service_level               service_level,
    is_dummy                    boolean,
    minimum_radius              numeric(9, 2),
    coupling_socket             socket_type,
    close_couplers              feature_flag,
    digital_shunting_coupling   feature_flag,
    flywheel_fitted             feature_flag,
    metal_body                  feature_flag,
    interior_lights             feature_flag,
    lights                      feature_flag,
    spring_buffers              feature_flag,
    CONSTRAINT "PK_rolling_stocks" PRIMARY KEY (rolling_stock_id),
    CONSTRAINT "FK_rolling_stocks_catalog_items" FOREIGN KEY (catalog_item_id)
        REFERENCES public.catalog_items (catalog_item_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_rolling_stocks_railways" FOREIGN KEY (railway_id)
        REFERENCES public.railways (railway_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);