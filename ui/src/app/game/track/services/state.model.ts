import { Phase } from './state.enum'
import { Game } from '../../../models/game'

export interface State {
    phase: Phase,
    game?: Game
}