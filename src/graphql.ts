
/** ------------------------------------------------------
 * THIS FILE WAS AUTOMATICALLY GENERATED (DO NOT MODIFY)
 * -------------------------------------------------------
 */

/* tslint:disable */
/* eslint-disable */
export abstract class IQuery {
    abstract line(id: string): Line | Promise<Line>;

    abstract station(id: string): Station | Promise<Station>;

    abstract stationByGroupId(groupId: string): Station | Promise<Station>;

    abstract stationByCoords(latitude: number, longitude: number): Station | Promise<Station>;

    abstract nearbyStations(latitude: number, longitude: number, limit?: number): Station[] | Promise<Station[]>;

    abstract stationsByLineId(lineId: string): Station[] | Promise<Station[]>;

    abstract stationsByName(name: string): Station[] | Promise<Station[]>;

    abstract allStations(): StationOnly[] | Promise<StationOnly[]>;

    abstract random(): Station | Promise<Station>;

    abstract trainType(id: string, excludePass?: boolean): TrainType | Promise<TrainType>;
}

export class Line {
    id?: number;
    companyId?: number;
    latitude?: number;
    longitude?: number;
    lineColorC?: string;
    lineColorT?: string;
    name?: string;
    nameH?: string;
    nameK?: string;
    nameR?: string;
    nameZh?: string;
    nameKo?: string;
    lineType?: number;
    zoom?: number;
    company?: Company;
}

export class Company {
    id?: number;
    railroadId?: number;
    name?: string;
    nameK?: string;
    nameH?: string;
    nameR?: string;
    nameEn?: string;
    url?: string;
    companyType?: number;
}

export class Station {
    id?: number;
    address?: string;
    distance?: number;
    latitude?: number;
    longitude?: number;
    lines?: Line[];
    currentLine?: Line;
    openYmd?: string;
    postalCode?: string;
    prefId?: number;
    groupId?: number;
    name?: string;
    nameK?: string;
    nameR?: string;
    nameZh?: string;
    nameKo?: string;
    trainTypes?: TrainType[];
    pass?: boolean;
}

export class StationOnly {
    id?: number;
    address?: string;
    distance?: number;
    latitude?: number;
    longitude?: number;
    openYmd?: string;
    postalCode?: string;
    prefId?: number;
    groupId?: number;
    name?: string;
    nameK?: string;
    nameR?: string;
    nameZh?: string;
    nameKo?: string;
}

export class TrainTypeMinimum {
    id?: number;
    typeId?: number;
    groupId?: number;
    name?: string;
    nameK?: string;
    nameR?: string;
    nameZh?: string;
    nameKo?: string;
    color?: string;
    line?: Line;
}

export class TrainType {
    id?: number;
    typeId?: number;
    groupId?: number;
    name?: string;
    nameK?: string;
    nameR?: string;
    nameZh?: string;
    nameKo?: string;
    color?: string;
    stations?: Station[];
    lines?: Line[];
    allTrainTypes?: TrainTypeMinimum[];
}
