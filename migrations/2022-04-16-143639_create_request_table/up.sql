create table request (
    id              integer     not null    primary key     autoincrement,
    route           text        not null,
    method          text        not null    check(method in ('get', 'post', 'delete', 'put', 'patch')),
    body            text,
    resource_id     integer     not null,
    
    foreign key (resource_id) references resource (id)
);
