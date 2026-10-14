DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns
               WHERE table_schema = 'performance' AND table_name = 'talent_matrix_entries' AND column_name = 'box_label') THEN
        ALTER TABLE performance.talent_matrix_entries RENAME COLUMN box_label TO box;
    END IF;
END $$;
