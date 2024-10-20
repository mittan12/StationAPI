use async_trait::async_trait;
use sqlx::{MySql, MySqlConnection, Pool};

use crate::domain::{
    entity::line::Line, error::DomainError, repository::line_repository::LineRepository,
};

#[derive(sqlx::FromRow, Clone)]
pub struct LineRow {
    pub line_cd: u32,
    pub company_cd: u32,
    pub line_name: String,
    pub line_name_k: String,
    pub line_name_h: String,
    pub line_name_r: Option<String>,
    pub line_name_zh: Option<String>,
    pub line_name_ko: Option<String>,
    pub line_color_c: String,
    pub line_type: u32,
    pub line_symbol_primary: Option<String>,
    pub line_symbol_secondary: Option<String>,
    pub line_symbol_extra: Option<String>,
    pub line_symbol_primary_color: Option<String>,
    pub line_symbol_secondary_color: Option<String>,
    pub line_symbol_extra_color: Option<String>,
    pub line_symbol_primary_shape: Option<String>,
    pub line_symbol_secondary_shape: Option<String>,
    pub line_symbol_extra_shape: Option<String>,
    pub e_status: u32,
    pub e_sort: u32,
    #[sqlx(default)]
    pub line_group_cd: Option<u32>,
    #[sqlx(default)]
    pub station_g_cd: Option<u32>,
    average_distance: f64,
}

impl From<LineRow> for Line {
    fn from(row: LineRow) -> Self {
        Self {
            line_cd: row.line_cd,
            company_cd: row.company_cd,
            company: None,
            line_name: row.line_name,
            line_name_k: row.line_name_k,
            line_name_h: row.line_name_h,
            line_name_r: row.line_name_r,
            line_name_zh: row.line_name_zh,
            line_name_ko: row.line_name_ko,
            line_color_c: row.line_color_c,
            line_type: row.line_type,
            line_symbols: vec![],
            line_symbol_primary: row.line_symbol_primary,
            line_symbol_secondary: row.line_symbol_secondary,
            line_symbol_extra: row.line_symbol_extra,
            line_symbol_primary_color: row.line_symbol_primary_color,
            line_symbol_secondary_color: row.line_symbol_secondary_color,
            line_symbol_extra_color: row.line_symbol_extra_color,
            line_symbol_primary_shape: row.line_symbol_primary_shape,
            line_symbol_secondary_shape: row.line_symbol_secondary_shape,
            line_symbol_extra_shape: row.line_symbol_extra_shape,
            e_status: row.e_status,
            e_sort: row.e_sort,
            station: None,
            train_type: None,
            line_group_cd: row.line_group_cd,
            station_g_cd: row.station_g_cd,
            average_distance: row.average_distance,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MyLineRepository {
    pool: Pool<MySql>,
}

impl MyLineRepository {
    pub fn new(pool: Pool<MySql>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LineRepository for MyLineRepository {
    async fn find_by_id(&self, id: u32) -> Result<Option<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::find_by_id(id, &mut conn).await
    }
    async fn find_by_station_id(&self, station_id: u32) -> Result<Option<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::find_by_station_id(station_id, &mut conn).await
    }
    async fn get_by_ids(&self, ids: Vec<u32>) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_ids(ids, &mut conn).await
    }
    async fn get_by_station_group_id(&self, id: u32) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_station_group_id(id, &mut conn).await
    }
    async fn get_by_station_group_id_vec(
        &self,
        station_group_id_vec: Vec<u32>,
    ) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_station_group_id_vec(station_group_id_vec, &mut conn).await
    }
    async fn get_by_line_group_id(&self, line_group_id: u32) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_line_group_id(line_group_id, &mut conn).await
    }
    async fn get_by_line_group_id_vec(
        &self,
        line_group_id_vec: Vec<u32>,
    ) -> Result<Vec<Line>, DomainError> {
        let mut conn = self.pool.acquire().await?;
        InternalLineRepository::get_by_line_group_id_vec(line_group_id_vec, &mut conn).await
    }
}

pub struct InternalLineRepository {}

impl InternalLineRepository {
    async fn find_by_id(id: u32, conn: &mut MySqlConnection) -> Result<Option<Line>, DomainError> {
        let rows: Option<LineRow> =
            sqlx::query_as("SELECT * FROM `lines` WHERE line_cd = ? AND e_status = 0")
                .bind(id)
                .fetch_optional(conn)
                .await?;
        let line: Option<Line> = rows.map(|row| row.into());

        let Some(line) = line else {
            return Ok(None);
        };

        Ok(Some(line))
    }

    async fn find_by_station_id(
        station_id: u32,
        conn: &mut MySqlConnection,
    ) -> Result<Option<Line>, DomainError> {
        let rows: Option<LineRow> = sqlx::query_as(
            "SELECT l.*,
            COALESCE(a.line_name, l.line_name) AS line_name,
            COALESCE(a.line_name_k, l.line_name_k) AS line_name_k,
            COALESCE(a.line_name_h, l.line_name_h) AS line_name_h,
            COALESCE(a.line_name_r, l.line_name_r) AS line_name_r,
            COALESCE(a.line_name_zh,l.line_name_zh) AS line_name_zh,
            COALESCE(a.line_name_ko, l.line_name_ko) AS line_name_ko,
            COALESCE(a.line_color_c, l.line_color_c) AS line_color_c
            FROM `lines` AS l
            LEFT OUTER JOIN `line_aliases` AS la
            ON
                la.station_cd = ?
            LEFT OUTER JOIN `aliases` AS a
            ON
                la.alias_cd = a.id                    
            WHERE l.line_cd IN (
                SELECT line_cd
                FROM stations AS s
                WHERE s.station_cd = ?
                AND s.e_status = 0
            )
            AND e_status = 0",
        )
        .bind(station_id)
        .bind(station_id)
        .fetch_optional(conn)
        .await?;
        let line: Option<Line> = rows.map(|row| row.into());

        let Some(line) = line else {
            return Ok(None);
        };

        Ok(Some(line))
    }

    async fn get_by_ids(
        ids: Vec<u32>,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Line>, DomainError> {
        if ids.is_empty() {
            return Ok(vec![]);
        }

        let params = format!("?{}", ", ?".repeat(ids.len() - 1));
        let query_str = format!(
            "SELECT * FROM `lines` WHERE line_cd IN ( {} ) AND e_status = 0",
            params
        );

        let mut query = sqlx::query_as::<_, LineRow>(&query_str);
        for id in ids {
            query = query.bind(id);
        }

        let rows = query.fetch_all(conn).await?;
        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();

        Ok(lines)
    }

    async fn get_by_station_group_id(
        station_group_id: u32,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Line>, DomainError> {
        let rows: Vec<LineRow> = sqlx::query_as(
            "SELECT 
            DISTINCT l.*,
            COALESCE(a.line_name, l.line_name) AS line_name,
            COALESCE(a.line_name_k, l.line_name_k) AS line_name_k,
            COALESCE(a.line_name_h, l.line_name_h) AS line_name_h,
            COALESCE(a.line_name_r, l.line_name_r) AS line_name_r,
            COALESCE(a.line_name_zh, l.line_name_zh) AS line_name_zh,
            COALESCE(a.line_name_ko, l.line_name_ko) AS line_name_ko,
            COALESCE(a.line_color_c, l.line_color_c) AS line_color_c,
            (
              SELECT 
                COUNT(sst.line_group_cd) 
              FROM 
                station_station_types AS sst 
              WHERE 
                s.station_cd = sst.station_cd 
                AND sst.pass <> 1
            ) AS station_types_count 
          FROM 
            (`lines` AS l, `stations` AS s) 
            LEFT OUTER JOIN `line_aliases` AS la ON la.station_cd = s.station_cd 
            LEFT OUTER JOIN `aliases` AS a ON la.alias_cd = a.id 
          WHERE 
            s.station_g_cd = ? 
            AND s.line_cd = l.line_cd
            AND l.e_status = 0
            AND s.e_status = 0",
        )
        .bind(station_group_id)
        .fetch_all(conn)
        .await?;
        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();

        Ok(lines)
    }

    async fn get_by_station_group_id_vec(
        station_group_id_vec: Vec<u32>,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Line>, DomainError> {
        if station_group_id_vec.is_empty() {
            return Ok(vec![]);
        }

        let params = format!("?{}", ", ?".repeat(station_group_id_vec.len() - 1));
        let query_str = format!(
            "SELECT 
            l.*,
            s.station_g_cd,
            COALESCE(a.line_name, l.line_name) AS line_name,
            COALESCE(a.line_name_k, l.line_name_k) AS line_name_k,
            COALESCE(a.line_name_h, l.line_name_h) AS line_name_h,
            COALESCE(a.line_name_r, l.line_name_r) AS line_name_r,
            COALESCE(a.line_name_zh, l.line_name_zh) AS line_name_zh,
            COALESCE(a.line_name_ko, l.line_name_ko) AS line_name_ko,
            COALESCE(a.line_color_c, l.line_color_c) AS line_color_c,
            (
              SELECT 
                COUNT(sst.line_group_cd) 
              FROM 
                station_station_types AS sst 
              WHERE 
                s.station_cd = sst.station_cd 
                AND sst.pass <> 1
            ) AS station_types_count 
          FROM 
            (
              `lines` AS l, `stations` AS s
            ) 
            LEFT OUTER JOIN `line_aliases` AS la ON la.station_cd = s.station_cd 
            LEFT OUTER JOIN `aliases` AS a ON la.alias_cd = a.id 
          WHERE 
            s.station_g_cd IN ( {} ) 
            AND s.line_cd = l.line_cd
            AND l.e_status = 0
            AND s.e_status = 0",
            params
        );

        let mut query = sqlx::query_as::<_, LineRow>(&query_str);
        for id in &station_group_id_vec {
            query = query.bind(id);
        }

        let rows = query.fetch_all(conn).await?;
        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();

        Ok(lines)
    }

    async fn get_by_line_group_id(
        line_group_id: u32,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Line>, DomainError> {
        let rows: Vec<LineRow> = sqlx::query_as(
            "SELECT 
            DISTINCT l.*,
            sst.line_group_cd,
            COALESCE(a.line_name, l.line_name) AS line_name,
            COALESCE(a.line_name_k, l.line_name_k) AS line_name_k,
            COALESCE(a.line_name_h, l.line_name_h) AS line_name_h,
            COALESCE(a.line_name_r, l.line_name_r) AS line_name_r,
            COALESCE(a.line_name_zh, l.line_name_zh) AS line_name_zh,
            COALESCE(a.line_name_ko, l.line_name_ko) AS line_name_ko,
            COALESCE(a.line_color_c, l.line_color_c) AS line_color_c
          FROM 
            (
              `lines` AS l, `stations` AS s, `station_station_types` AS sst
            ) 
            LEFT OUTER JOIN `line_aliases` AS la ON la.station_cd = s.station_cd 
            LEFT OUTER JOIN `aliases` AS a ON la.alias_cd = a.id 
          WHERE 
            sst.line_group_cd = ?
            AND sst.station_cd = s.station_cd
            AND l.e_status = 0
            AND s.line_cd = l.line_cd
            AND s.e_status = 0",
        )
        .bind(line_group_id)
        .fetch_all(conn)
        .await?;
        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();
        Ok(lines)
    }

    async fn get_by_line_group_id_vec(
        line_group_id_vec: Vec<u32>,
        conn: &mut MySqlConnection,
    ) -> Result<Vec<Line>, DomainError> {
        if line_group_id_vec.is_empty() {
            return Ok(vec![]);
        }

        let params = format!("?{}", ", ?".repeat(line_group_id_vec.len() - 1));
        let query_str = format!(
            "SELECT
            DISTINCT l.*,
            sst.line_group_cd,
            COALESCE(a.line_name, l.line_name) AS line_name,
            COALESCE(a.line_name_k, l.line_name_k) AS line_name_k,
            COALESCE(a.line_name_h, l.line_name_h) AS line_name_h,
            COALESCE(a.line_name_r, l.line_name_r) AS line_name_r,
            COALESCE(a.line_name_zh, l.line_name_zh) AS line_name_zh,
            COALESCE(a.line_name_ko, l.line_name_ko) AS line_name_ko,
            COALESCE(a.line_color_c, l.line_color_c) AS line_color_c
          FROM
            (
              `lines` AS l, `stations` AS s, `station_station_types` AS sst
            )
            LEFT OUTER JOIN `line_aliases` AS la ON la.station_cd = s.station_cd
            LEFT OUTER JOIN `aliases` AS a ON la.alias_cd = a.id
          WHERE
            sst.line_group_cd IN ( {} )
            AND sst.station_cd = s.station_cd
            AND s.line_cd = l.line_cd
            AND l.e_status = 0
            AND s.e_status = 0",
            params
        );

        let mut query = sqlx::query_as::<_, LineRow>(&query_str);
        for id in line_group_id_vec {
            query = query.bind(id);
        }

        let rows = query.fetch_all(conn).await?;
        let lines: Vec<Line> = rows.into_iter().map(|row| row.into()).collect();

        Ok(lines)
    }
}
