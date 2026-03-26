INSERT INTO railways (
    railway_id, name, abbreviation, registered_company_name, organization_entity_type,
    description_de, description_en, description_fr, description_it,
    country, operating_since, operating_until, status,
    gauge_meters, track_gauge, headquarters,
    total_length_km, total_length_mi,
    contact_email, contact_website_url, contact_phone,
    socials_facebook, socials_instagram, socials_linkedin, socials_twitter, socials_youtube,
    created_at, version
) VALUES (
    'fs', 'FS', 'FS', 'Ferrovie dello Stato Italiane S.p.A.', 'STATE_OWNED_ENTERPRISE',
    'beschreibung', 'description', 'description', 'descrizione',
    'IT', '1905-07-01', NULL, 'ACTIVE',
    1.435, 'STANDARD', ARRAY['Roma'],
    24564.0, 15263.4,
    'mail@mail.com', 'https://www.fsitaliane.it', '+14152370800',
    'fsitaliane', 'fsitaliane', 'ferrovie-dello-stato-s-p-a-', 'FSitaliane', 'fsitaliane',
    NOW(), 1
),
(
    'db', 'DB', 'DB', 'Deutsche Bahn AG', 'STATE_OWNED_ENTERPRISE',
    'beschreibung', 'description', 'description', 'descrizione',
    'DE', '1994-01-01', NULL, 'ACTIVE',
    1.435, 'STANDARD', ARRAY['Berlin'],
    NULL, NULL,
    NULL, 'https://www.deutschebahn.com', NULL,
    NULL, 'deutschebahn', 'deutschebahn', 'db_presse', 'deutschebahnkonzern',
    NOW(), 1
);
