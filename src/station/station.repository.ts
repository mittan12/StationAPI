import { Injectable } from '@nestjs/common';
import { RowDataPacket } from 'mysql2';
import { NEX_ID } from 'src/constants/ignore';
import { TrainType } from 'src/graphql';
import { LineRepository } from 'src/line/line.repository';
import { MysqlService } from 'src/mysql/mysql.service';
import { TrainTypeRepository } from 'src/trainType/trainType.repository';
import { StationRaw } from './models/StationRaw';
import { uniqBy } from 'lodash';

@Injectable()
export class StationRepository {
  constructor(
    private readonly mysqlService: MysqlService,
    private readonly lineRepo: LineRepository,
    private readonly trainTypeRepo: TrainTypeRepository,
  ) {}

  async findOneById(id: number): Promise<StationRaw> {
    const { connection } = this.mysqlService;

    return new Promise<StationRaw>((resolve, reject) => {
      connection.query(
        `
          SELECT *
          FROM stations
          WHERE station_cd = ?
          AND e_status = 0
          AND NOT line_cd = ${NEX_ID}
        `,
        [id],
        async (err, results: RowDataPacket[]) => {
          if (err) {
            return reject(err);
          }
          if (!results.length) {
            return resolve(null);
          }

          const lines = await this.lineRepo.getByGroupId(
            results[0].station_g_cd,
          );
          return resolve({
            ...(results[0] as StationRaw),
            lines,
          });
        },
      );
    });
  }

  async findTrainTypesById(id: number): Promise<TrainType[]> {
    const { connection } = this.mysqlService;

    return new Promise<TrainType[]>((resolve, reject) => {
      connection.query(
        `
          SELECT sst.type_cd,
            sst.id,
            sst.line_group_cd,
            t.type_name,
            t.type_name_k,
            t.type_name_r,
            t.type_name_zh,
            t.type_name_ko,
            t.color,
            l.line_cd
          FROM station_station_types as sst,
            types as t,
            stations as s,
            \`lines\` as l
          WHERE sst.station_cd = ?
            AND sst.type_cd = t.type_cd
            AND sst.station_cd = s.station_cd
            AND l.line_cd = s.line_cd
            AND sst.pass = 0
        `,
        [id],
        async (err, results: RowDataPacket[]) => {
          if (err) {
            return reject(err);
          }
          if (!results.length) {
            return resolve([]);
          }

          const parenthesisRegexp = /\([^()]*\)/g;

          return resolve(
            Promise.all(
              results.map(
                async (r): Promise<TrainType> => {
                  const allTrainTypes = await this.trainTypeRepo.getAllLinesTrainTypes(
                    r.line_group_cd,
                  );

                  const duplicatedTrainTypes = allTrainTypes.filter(
                    (tt, i, arr) =>
                      arr.findIndex(
                        (item) => item.company_cd === tt.company_cd,
                      ) !== i,
                  );

                  const duplicatedCompanies = duplicatedTrainTypes.length
                    ? await Promise.all(
                        duplicatedTrainTypes.map(
                          async (tt) =>
                            await this.lineRepo.findOneCompany(tt.line_cd),
                        ),
                      )
                    : [];

                  const filteredAllTrainTypes = allTrainTypes.filter(
                    (tt) => tt.line_cd !== r.line_cd,
                  );

                  const notDuplicatedTypes = filteredAllTrainTypes.filter(
                    (tt) => {
                      if (!duplicatedCompanies.length) {
                        return tt;
                      }
                      return duplicatedCompanies.find(
                        (dc) => dc.company_cd !== tt.company_cd,
                      );
                    },
                  );

                  const joinedName = (() => {
                    if (duplicatedCompanies.length) {
                      // return [...notDuplicatedTypes, ...duplicatedTrainTypes];
                      return `${r.type_name}(${notDuplicatedTypes
                        .map(
                          (tt) =>
                            `${tt.line_name.replace(
                              parenthesisRegexp,
                              '',
                            )}内${tt.type_name.replace(parenthesisRegexp, '')}`,
                        )
                        .join('/')}/${duplicatedCompanies
                        .map(
                          (dc, i) =>
                            `${dc.company_name_r}線内${duplicatedTrainTypes[i].type_name}`,
                        )
                        .join('/')})`;
                    }
                    return `${r.type_name}(${filteredAllTrainTypes
                      .map(
                        (tt) =>
                          `${tt.line_name.replace(
                            parenthesisRegexp,
                            '',
                          )}内${tt.type_name.replace(parenthesisRegexp, '')}`,
                      )
                      .join('/')})`;
                  })();
                  const joinedNameR = (() => {
                    if (duplicatedCompanies.length) {
                      // return [...notDuplicatedTypes, ...duplicatedTrainTypes];
                      return `${r.type_name_r}(${notDuplicatedTypes
                        .map(
                          (tt) =>
                            `${tt.line_name_r.replace(
                              parenthesisRegexp,
                              '',
                            )} ${tt.type_name_r.replace(
                              parenthesisRegexp,
                              '',
                            )}`,
                        )
                        .join('/')}/${duplicatedCompanies
                        .map(
                          (dc, i) =>
                            `${dc.company_name_en} Line ${duplicatedTrainTypes[i].type_name_r}`,
                        )
                        .join('/')})`;
                    }
                    return `${r.type_name_r}(${filteredAllTrainTypes
                      .map(
                        (tt) =>
                          `${tt.line_name_r.replace(
                            parenthesisRegexp,
                            '',
                          )} ${tt.type_name_r.replace(parenthesisRegexp, '')}`,
                      )
                      .join('/')})`;
                  })();

                  return {
                    // キャッシュが重複しないようにするため。もっとうまい方法あると思う
                    id: r.id + r.type_cd + r.line_group_cd,
                    typeId: r.type_cd,
                    groupId: r.line_group_cd,
                    name: !filteredAllTrainTypes.length
                      ? r.type_name
                      : joinedName,
                    nameK: r.type_name_k,
                    nameR: !filteredAllTrainTypes.length
                      ? r.type_name_r
                      : joinedNameR,
                    nameZh: r.type_name_zh,
                    nameKo: r.type_name_ko,
                    color: r.color,
                    allTrainTypes: allTrainTypes.map((tt) => ({
                      id: tt.type_cd + tt.line_cd,
                      typeId: tt.type_cd,
                      groupId: r.line_group_cd,
                      name: tt.type_name,
                      nameK: tt.type_name_k,
                      nameR: tt.type_name_r,
                      nameZh: tt.type_name_zh,
                      nameKo: tt.type_name_ko,
                      color: tt.color,
                      line: {
                        id: tt.line_cd,
                        companyId: tt.company_cd,
                        latitude: tt.lat,
                        longitude: tt.lon,
                        lineColorC: tt.line_color_c,
                        lineColorT: tt.line_color_t,
                        name: tt.line_name,
                        nameH: tt.line_name_h,
                        nameK: tt.line_name_k,
                        nameR: tt.type_name_r,
                        nameZh: tt.line_name_zh,
                        nameKo: tt.line_name_ko,
                        lineType: tt.line_type,
                        zoom: tt.zoom,
                      },
                    })),
                    lines: await this.trainTypeRepo.getBelongingLines(
                      r.line_group_cd,
                    ),
                  };
                },
              ),
            ),
          );
        },
      );
    });
  }

  async findOneByGroupId(groupId: number): Promise<StationRaw> {
    const { connection } = this.mysqlService;

    return new Promise<StationRaw>((resolve, reject) => {
      connection.query(
        `
          SELECT *
          FROM stations
          WHERE station_g_cd = ?
          AND e_status = 0
          AND NOT line_cd = ${NEX_ID}
        `,
        [groupId],
        async (err, results: RowDataPacket[]) => {
          if (err) {
            return reject(err);
          }
          if (!results.length) {
            return resolve(null);
          }
          const lines = await this.lineRepo.getByGroupId(
            results[0].station_g_cd,
          );
          return resolve({
            ...(results[0] as StationRaw),
            lines,
          });
        },
      );
    });
  }

  async findOneByCoords(
    latitude: number,
    longitude: number,
  ): Promise<StationRaw> {
    const { connection } = this.mysqlService;

    return new Promise<StationRaw>((resolve, reject) => {
      connection.query(
        `
        SELECT *,
        (
          6371 * acos(
          cos(radians(?))
          * cos(radians(lat))
          * cos(radians(lon) - radians(?))
          + sin(radians(?))
          * sin(radians(lat))
          )
        ) AS distance
        FROM
        stations
        WHERE
        e_status = 0
        ORDER BY
        distance
        LIMIT 1
        `,
        [latitude, longitude, latitude],
        async (err, results: RowDataPacket[]) => {
          if (err) {
            return reject(err);
          }
          if (!results.length) {
            return resolve(null);
          }
          const lines = await this.lineRepo.getByGroupId(
            results[0].station_g_cd,
          );
          return resolve({
            ...(results[0] as StationRaw),
            lines,
          });
        },
      );
    });
  }

  async getByLineId(lineId: number): Promise<StationRaw[]> {
    const { connection } = this.mysqlService;
    if (!connection) {
      return [];
    }

    return new Promise<StationRaw[]>((resolve, reject) => {
      connection.query(
        `
          SELECT *
          FROM stations
          WHERE line_cd = ?
          AND e_status = 0
          AND NOT line_cd = ${NEX_ID}
          ORDER BY e_sort, station_cd
        `,
        [lineId],
        async (err, results: RowDataPacket[]) => {
          if (err) {
            return reject(err);
          }
          if (!results.length) {
            return resolve([]);
          }

          const map = await Promise.all<StationRaw>(
            results.map(async (r) => {
              const lines = await this.lineRepo.getByGroupId(r.station_g_cd);
              return {
                ...r,
                lines,
              } as StationRaw;
            }),
          );

          return resolve(map);
        },
      );
    });
  }

  async getByName(name: string): Promise<StationRaw[]> {
    const { connection } = this.mysqlService;
    if (!connection) {
      return [];
    }

    return new Promise<StationRaw[]>((resolve, reject) => {
      connection.query(
        `
          SELECT * FROM stations
          WHERE (station_name LIKE "%${name}%"
          OR station_name_r LIKE "%${name}%"
          OR station_name_k LIKE "%${name}%"
          OR station_name_zh LIKE "%${name}%"
          OR station_name_ko LIKE "%${name}%")
          AND e_status = 0
          AND NOT line_cd = ${NEX_ID}
          ORDER BY e_sort, station_cd
        `,
        [],
        async (err, results: RowDataPacket[]) => {
          if (err) {
            return reject(err);
          }
          if (!results.length) {
            return resolve([]);
          }

          const map = await Promise.all<StationRaw>(
            results.map(async (r) => {
              const line = await this.lineRepo.findOne(r.line_cd);
              return {
                ...r,
                lines: [line],
              } as StationRaw;
            }),
          );

          return resolve(map);
        },
      );
    });
  }

  async getAll(): Promise<StationRaw[]> {
    const { connection } = this.mysqlService;

    return new Promise<StationRaw[]>((resolve, reject) => {
      connection.query(
        `
          SELECT *
          FROM stations
          WHERE e_status = 0
          AND NOT line_cd = ${NEX_ID}
          AND station_g_cd IN (
            SELECT DISTINCT station_g_cd
            FROM stations
          )
        `,
        [],
        async (err, results: RowDataPacket[]) => {
          if (err) {
            return reject(err);
          }
          if (!results.length) {
            return resolve([]);
          }
          const filtered = uniqBy(results, 'station_g_cd');
          return resolve(filtered as StationRaw[]);
        },
      );
    });
  }
}
