CREATE TABLE account
(
  account_id oid NOT NULL,
  username varchar(20) NOT NULL,
  password varchar(128) NOT NULL,
  display_name varchar(40) NOT NULL,
  identity_public_key varchar(64) NOT NULL,
  signed_prekey varchar(64) NOT NULL,
  prekeys jsonb NOT NULL,
  is_active boolean DEFAULT TRUE NOT NULL,
  registeration_timestamp timestamp NOT NULL
);

ALTER TABLE account ADD CONSTRAINT pk_account
  PRIMARY KEY (account_id);

CREATE TABLE delivery_queue
(
  id oid NOT NULL,
  recipient_id text NOT NULL,
  contents json NOT NULL
);

ALTER TABLE delivery_queue ADD CONSTRAINT pk_delivery_queue
  PRIMARY KEY (id);
