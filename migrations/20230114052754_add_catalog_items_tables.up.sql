-- noinspection SqlNoDataSourceInspectionForFile
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
CREATE TYPE body_shell_type AS ENUM (
    'METAL_DIE_CAST',
    'PLASTIC'
    );
CREATE TYPE chassis_type AS ENUM (
    'METAL_DIE_CAST',
    'PLASTIC'
    );

CREATE TABLE IF NOT EXISTS public.catalog_items
(
    catalog_item_id     varchar(76)           NOT NULL,
    manufacturer_id            varchar(50)           NOT NULL,
    item_number         varchar(25)           NOT NULL,
    scale_id            varchar(25)           NOT NULL,
    category            catalog_item_category NOT NULL,
    description_de      varchar(2500),
    description_en      varchar(2500),
    description_fr      varchar(2500),
    description_it      varchar(2500),
    details_de          varchar(2500),
    details_en          varchar(2500),
    details_fr          varchar(2500),
    details_it          varchar(2500),
    power_method        power_method          NOT NULL,
    epoch               varchar(10)           NOT NULL,
    delivery_date       varchar(10),
    availability_status availability_status,
    count               integer               NOT NULL DEFAULT 1,
    created_at          timestamptz           NOT NULL,
    last_modified_at    timestamptz,
    version             integer               NOT NULL DEFAULT 1,
    CONSTRAINT "PK_catalog_items" PRIMARY KEY (catalog_item_id),
    CONSTRAINT "FK_catalog_items_manufacturers" FOREIGN KEY (manufacturer_id)
        REFERENCES public.manufacturers (manufacturer_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION,
    CONSTRAINT "FK_catalog_items_scales" FOREIGN KEY (scale_id)
        REFERENCES public.scales (scale_id) MATCH SIMPLE
        ON UPDATE NO ACTION
        ON DELETE NO ACTION
);

CREATE TABLE IF NOT EXISTS public.rolling_stocks
(
    rolling_stock_id            uuid                   NOT NULL,
    catalog_item_id             varchar(65)            NOT NULL,
    railway_id                  varchar(25)            NOT NULL,
    rolling_stock_category      rolling_stock_category NOT NULL,
    livery                      varchar(50),
    length_over_buffers_mm      numeric(9, 2),
    length_over_buffers_in      numeric(9, 2),
    type_name                   varchar(25)            NOT NULL,
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
    body_shell                  body_shell_type,
    chassis                     chassis_type,
    interior_lights             feature_flag,
    lights                      feature_flag,
    sprung_buffers              feature_flag,
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
