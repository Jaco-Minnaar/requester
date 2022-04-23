create table param (
    id          integer     not null    primary key     autoincrement,
    key         text        not null,
    value       text        not null,
    request_id  integer     not null,
    foreign key (request_id) references request (id)
)
