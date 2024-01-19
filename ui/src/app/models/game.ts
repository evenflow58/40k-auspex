import { Player } from './player';

export class Game {
    constructor(
        public id: string,
        public name: string,
        public attacker: Player,
        public defender: Player,
        public size: string,
    ) { }
}