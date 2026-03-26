INSERT INTO brands (
    brand_id, name, registered_company_name, organization_entity_type, group_name,
    description_de, description_en, description_fr, description_it,
    kind, status,
    contact_email, contact_website_url, contact_phone,
    address_street_address, address_extended_address, address_city, address_region, address_postal_code, address_country,
    socials_facebook, socials_instagram, socials_linkedin, socials_twitter, socials_youtube,
    created_at, version
) VALUES (
    'acme', 'ACME', 'Associazione Costruzioni Modellistiche Esatte', 'LIMITED_COMPANY', 'UNKNOWN',
    'beschreibung', 'description', 'description', 'descrizione',
    'INDUSTRIAL', 'ACTIVE',
    'mail@acmetreni.com', 'http://www.acmetreni.com', '+39029867556',
    'Viale Lombardia, 27', 'Interno 42', 'Milano', 'MI', '20131', 'IT',
    'facebook_handler', 'instagram_handler', 'linkedin_handler', 'twitter_handler', 'youtube_handler',
    NOW(), 1
),
(
    'piko', 'PIKO', 'PIKO Spielwaren GmbH', 'LIMITED_COMPANY', NULL,
    'beschreibung', 'description', 'description', 'descrizione',
    'INDUSTRIAL', 'ACTIVE',
    'info@piko.de', 'https://www.piko.de', NULL,
    'Lutherstraße 30', NULL, 'Sonneberg', NULL, 'D-96515', 'DE',
    'PIKO.Modellbahn', 'piko.modellbahn', NULL, NULL, 'pikode',
    NOW(), 1
);
