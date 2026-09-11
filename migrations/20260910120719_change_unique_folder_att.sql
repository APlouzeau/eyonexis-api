alter table folders drop constraint folders_folder_name_key;
alter table folders drop constraint folders_folder_slug_key;
alter table folders add constraint parent_id_folder_slug_unique unique (parent_id, folder_name, folder_slug);
CREATE UNIQUE INDEX ON folders(folder_slug, folder_name) WHERE parent_id IS NULL;