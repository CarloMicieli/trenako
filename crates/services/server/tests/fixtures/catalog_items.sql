INSERT INTO catalog_items (
    catalog_item_id, brand_id, item_number, scale_id, category,
    description_de, description_en, description_fr, description_it,
    details_de, details_en, details_fr, details_it,
    power_method, epoch, delivery_date, availability_status, count,
    created_at, version
) VALUES (
    'acme-60011', 'acme', '60011', 'h0', 'LOCOMOTIVES',
    'Beschreibung auf Deutsch', 'English description', 'Description en anglais',
    'Locomotiva elettrica E 402A 015 nella livrea di origine rosso/bianco versione di origine, pantografi 52 Sommerfeldt',
    'einzelheiten', 'details', 'détails', 'dettagli',
    'DC', 'V', '2005', 'AVAILABLE', 1,
    NOW(), 1
);

INSERT INTO rolling_stocks (
    rolling_stock_id, catalog_item_id, railway_id, rolling_stock_category,
    livery, length_over_buffers_mm, length_over_buffers_in,
    type_name, road_number, series, depot,
    dcc_interface, control, locomotive_type,
    is_dummy,
    minimum_radius, coupling_socket, close_couplers, digital_shunting_coupling,
    flywheel_fitted, interior_lights, lights, sprung_buffers
) VALUES (
    gen_random_uuid(), 'acme-60011', 'fs', 'LOCOMOTIVE',
    'rosso/bianco', 210.0, 8.27,
    'E402 A', 'E402 015', NULL, 'Milano Centrale',
    'MTC_21', 'DCC_READY', 'ELECTRIC_LOCOMOTIVE',
    false,
    360.0, 'NEM_362', 'NO', 'NO',
    'NO', 'NO', 'YES', 'NO'
);
