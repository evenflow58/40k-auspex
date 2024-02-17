export class Player {
    constructor(
        public id: string,
        public name: string,
        public army: string,
        public armyList: string,
        public currentMission: string,
        public discardedMissions: Array<string>,
        public missionType: string,
        public turnOrder: string,
        public playerType: string,
    ) { }
}