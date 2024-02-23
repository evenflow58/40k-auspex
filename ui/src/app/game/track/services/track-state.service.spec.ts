import { TestBed } from '@angular/core/testing';

import { TrackStateService } from './track-state.service';

describe('TrackStateService', () => {
  let service: TrackStateService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(TrackStateService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
