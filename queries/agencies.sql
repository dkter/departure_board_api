--! insert_agency
INSERT INTO Agencies (Agency, checksum, timezone) VALUES (:agency, :checksum, :timezone);

--! delete_agency
DELETE FROM Agencies WHERE Agency = :agency;

--! get_agency_checksum
SELECT checksum FROM Agencies WHERE Agency = :agency;