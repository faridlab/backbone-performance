-- The talent matrix entry's 9-box label is declared as box_label; the table was created with
-- the column named box. Rename it so the entity reads and writes the column it declares.
DO $$
BEGIN
    IF EXISTS (SELECT 1 FROM information_schema.columns
               WHERE table_schema = 'performance' AND table_name = 'talent_matrix_entries' AND column_name = 'box')
       AND NOT EXISTS (SELECT 1 FROM information_schema.columns
               WHERE table_schema = 'performance' AND table_name = 'talent_matrix_entries' AND column_name = 'box_label') THEN
        ALTER TABLE performance.talent_matrix_entries RENAME COLUMN box TO box_label;
    END IF;
END $$;

-- The declared max-length check on box_label, skipped when it first ran because the column
-- had the other name.
DO $$
BEGIN
    IF to_regclass('performance.talent_matrix_entries') IS NOT NULL
       AND NOT EXISTS (SELECT 1 FROM pg_constraint WHERE conrelid = to_regclass('performance.talent_matrix_entries') AND conname = 'ck_talent_matrix_entries_box_label_max_length__max_length') THEN
        IF EXISTS (SELECT 1 FROM performance.talent_matrix_entries WHERE NOT ((char_length(box_label) <= 40))) THEN
            RAISE NOTICE 'ck_talent_matrix_entries_box_label_max_length__max_length not added: stored rows of performance.talent_matrix_entries break it';
        ELSE
            ALTER TABLE performance.talent_matrix_entries ADD CONSTRAINT ck_talent_matrix_entries_box_label_max_length__max_length CHECK ((char_length(box_label) <= 40));
        END IF;
    END IF;
EXCEPTION WHEN undefined_column THEN
    RAISE NOTICE 'ck_talent_matrix_entries_box_label_max_length__max_length not added: performance.talent_matrix_entries lacks a column it reads';
END $$;
