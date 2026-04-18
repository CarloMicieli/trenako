-- noinspection SqlNoDataSourceInspectionForFile
CREATE TYPE manufacturer_kind AS ENUM (
    'BRASS_MODELS',
    'INDUSTRIAL'
    );
CREATE TYPE manufacturer_status AS ENUM (
    'ACTIVE',
    'OUT_OF_BUSINESS'
    );
CREATE TYPE track_gauge AS ENUM (
    'BROAD',
    'MEDIUM',
    'MINIMUM',
    'NARROW',
    'STANDARD'
    );
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
CREATE TYPE railway_status AS ENUM (
    'ACTIVE',
    'INACTIVE'
    );
CREATE TYPE scale_standard AS ENUM (
    'BRITISH',
    'JAPANESE',
    'NEM',
    'NMRA'
    );

CREATE TABLE IF NOT EXISTS public.manufacturers
(
    manufacturer_id                 varchar(50) NOT NULL,
    name                     varchar(50) NOT NULL,
    registered_company_name  varchar(100),
    organization_entity_type organization_entity_type,
    group_name               varchar(100),
    description_de           varchar(2500),
    description_en           varchar(2500),
    description_fr           varchar(2500),
    description_it           varchar(2500),
    kind                     manufacturer_kind  NOT NULL,
    status                   manufacturer_status,
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
    CONSTRAINT "PK_manufacturers" PRIMARY KEY (manufacturer_id)
);

CREATE TABLE IF NOT EXISTS public.railways
(
    railway_id               varchar(50) NOT NULL,
    name                     varchar(50) NOT NULL,
    abbreviation             varchar(10),
    registered_company_name  varchar(250),
    organization_entity_type organization_entity_type,
    description_de           varchar(2500),
    description_en           varchar(2500),
    description_fr           varchar(2500),
    description_it           varchar(2500),
    country                  varchar(3)  NOT NULL,
    operating_since          date,
    operating_until          date,
    status                   railway_status,
    gauge_meters             numeric(5, 3),
    track_gauge              track_gauge,
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

CREATE TABLE IF NOT EXISTS public.scales
(
    scale_id          varchar(50)    NOT NULL,
    name              varchar(50)    NOT NULL,
    ratio             numeric(6, 2)  NOT NULL,
    gauge_millimeters numeric(4, 1),
    gauge_inches      numeric(5, 2),
    track_gauge       track_gauge    NOT NULL,
    description_de    varchar(2500),
    description_en    varchar(2500),
    description_fr    varchar(2500),
    description_it    varchar(2500),
    standards         scale_standard array,
    created_at        timestamptz    NOT NULL,
    last_modified_at  timestamptz,
    version           integer        NOT NULL DEFAULT 1,
    CONSTRAINT "PK_scales" PRIMARY KEY (scale_id)
);
