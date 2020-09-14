// Copyright 2019-2020 Twitter, Inc.
// Licensed under the Apache License, Version 2.0
// http://www.apache.org/licenses/LICENSE-2.0

use rustcommon_atomics::*;
use serde_derive::Deserialize;
use strum::IntoEnumIterator;

use crate::config::SamplerConfig;

use super::stat::*;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SchedulerConfig {
    #[serde(default)]
    bpf: AtomicBool,
    #[serde(default)]
    enabled: AtomicBool,
    #[serde(default)]
    interval: Option<AtomicUsize>,
    #[serde(default = "crate::common::default_percentiles")]
    percentiles: Vec<f64>,
    #[serde(default)]
    perf_events: AtomicBool,
    #[serde(default = "default_statistics")]
    statistics: Vec<SchedulerStatistic>,
}

impl Default for SchedulerConfig {
    fn default() -> Self {
        Self {
            bpf: Default::default(),
            enabled: Default::default(),
            interval: Default::default(),
            percentiles: crate::common::default_percentiles(),
            perf_events: Default::default(),
            statistics: default_statistics(),
        }
    }
}

fn default_statistics() -> Vec<SchedulerStatistic> {
    SchedulerStatistic::iter().collect()
}

impl SamplerConfig for SchedulerConfig {
    type Statistic = SchedulerStatistic;

    fn bpf(&self) -> bool {
        self.bpf.load(Ordering::Relaxed)
    }

    fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    fn interval(&self) -> Option<usize> {
        self.interval.as_ref().map(|v| v.load(Ordering::Relaxed))
    }

    fn percentiles(&self) -> &[f64] {
        &self.percentiles
    }

    fn perf_events(&self) -> bool {
        self.perf_events.load(Ordering::Relaxed)
    }

    fn statistics(&self) -> Vec<<Self as SamplerConfig>::Statistic> {
        let mut enabled = Vec::new();
        for statistic in self.statistics.iter() {
            if statistic.perf_table().is_some() {
                if self.perf_events() {
                    enabled.push(statistic.clone());
                }
            } else if statistic.bpf_table().is_some() {
                if self.bpf() {
                    enabled.push(statistic.clone());
                }
            } else {
                enabled.push(statistic.clone());
            }
        }
        enabled
    }
}
