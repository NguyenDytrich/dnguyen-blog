FROM postgres:13

COPY ["./migrations/initialize.sql", "/docker-entrypoint-initdb.d/"]
ENTRYPOINT ["docker-entrypoint.sh"]
EXPOSE 5432
CMD ["postgres"]
