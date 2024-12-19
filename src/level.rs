pub struct LevelConfig {
    pub fall_interval: f32,
    pub lines_required: u32,
    pub score_multiplier: f32,
}

pub const LEVEL_CONFIGS: [LevelConfig; 10] = [
    LevelConfig {
        fall_interval: 0.48,
        lines_required: 10,
        score_multiplier: 1.0,
    }, // Level 1
    LevelConfig {
        fall_interval: 0.40,
        lines_required: 20,
        score_multiplier: 1.5,
    }, // Level 2
    LevelConfig {
        fall_interval: 0.32,
        lines_required: 30,
        score_multiplier: 2.0,
    }, // Level 3
    LevelConfig {
        fall_interval: 0.24,
        lines_required: 40,
        score_multiplier: 2.5,
    }, // Level 4
    LevelConfig {
        fall_interval: 0.16,
        lines_required: 50,
        score_multiplier: 3.0,
    }, // Level 5
    LevelConfig {
        fall_interval: 0.14,
        lines_required: 60,
        score_multiplier: 3.5,
    }, // Level 6
    LevelConfig {
        fall_interval: 0.12,
        lines_required: 70,
        score_multiplier: 4.0,
    }, // Level 7
    LevelConfig {
        fall_interval: 0.11,
        lines_required: 80,
        score_multiplier: 4.5,
    }, // Level 8
    LevelConfig {
        fall_interval: 0.10,
        lines_required: 90,
        score_multiplier: 4.7,
    }, // Level 9
    LevelConfig {
        fall_interval: 0.10,
        lines_required: 100,
        score_multiplier: 5.0,
    }, // Level 10
];
