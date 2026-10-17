-- An appraisal cycle can be called off before it closes: the cancel verb
-- writes 'cancelled', which cycle_status did not carry, so the verb failed.
-- Enum labels can only be appended.
ALTER TYPE cycle_status ADD VALUE IF NOT EXISTS 'cancelled';
