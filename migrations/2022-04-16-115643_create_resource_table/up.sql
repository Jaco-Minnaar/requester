CREATE TABLE resource (
	id integer primary key autoincrement not null,
	name text not null,
	api_id integer not null,

	foreign key (api_id) references api (id)
)
