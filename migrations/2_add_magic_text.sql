alter table foo add column magic_text text;
update foo set magic_text = name;
alter table foo alter column magic_text set not null;