CREATE TABLE email_verification_tokens
(
    id         UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id    UUID NOT NULL,
    token      UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),

    CONSTRAINT fk_email_user
        FOREIGN KEY (user_id)
            REFERENCES users (id)
            ON DELETE CASCADE
);

CREATE INDEX idx_email_user_id ON email_verification_tokens (user_id);
CREATE INDEX idx_email_created_at ON email_verification_tokens (created_at);
