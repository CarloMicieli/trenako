INSERT INTO scales (
    scale_id, name, ratio, gauge_millimeters, gauge_inches, track_gauge,
    description_de, description_en, description_fr, description_it,
    standards, created_at, version
) VALUES (
    'h0', 'H0', 87.0, 16.5, 0.65, 'STANDARD',
    'beschreibung', 'description', 'description', 'descrizione',
    ARRAY['NEM']::scale_standard[], NOW(), 1
),
(
    'n', 'N', 160.0, 9.0, 0.354, 'STANDARD',
    'beschreibung', 'description', 'description', 'descrizione',
    ARRAY['NEM']::scale_standard[], NOW(), 1
);
