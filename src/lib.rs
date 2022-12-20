// -*- coding: utf-8 -*-

pub mod systime {
    use std::time::Duration;

    pub struct EspSystemTime;

    impl EspSystemTime {
        pub fn now(&self) -> Duration {
            Duration::from_millis(0)
        }
    }
}

pub mod timer {}

pub mod eventloop {
    use dummy_esp_idf_sys::EspError;
    use std::marker::PhantomData;

    pub struct System;

    pub struct EspEventLoop<T>(PhantomData<T>);
    pub type EspSystemEventLoop = EspEventLoop<System>;

    impl EspEventLoop<System> {
        pub fn take() -> Result<Self, EspError> {
            Ok(Self(PhantomData))
        }
    }
}

pub mod wifi {
    use super::eventloop::EspSystemEventLoop;
    use super::nvs::EspDefaultNvsPartition;
    use dummy_esp_idf_hal::{modem::WifiModemPeripheral, peripheral::Peripheral};
    use dummy_esp_idf_sys::EspError;
    use std::marker::PhantomData;

    pub struct EspWifi<'a>(PhantomData<&'a ()>);

    impl<'a> EspWifi<'a> {
        pub fn new<M: WifiModemPeripheral>(
            _: impl Peripheral<P = M> + 'a,
            _: EspSystemEventLoop,
            _: Option<EspDefaultNvsPartition>,
        ) -> Result<Self, EspError> {
            Ok(Self(PhantomData))
        }
    }
}

pub mod nvs {
    use dummy_esp_idf_sys::EspError;
    use std::marker::PhantomData;

    pub struct EspNvs<T>(PhantomData<T>);

    impl<T> EspNvs<T> {
        pub fn new(_: EspNvsPartition<T>, _: &str, _: bool) -> Result<Self, EspError> {
            Ok(Self(PhantomData))
        }

        pub fn contains(&self, _: &str) -> Result<bool, EspError> {
            Ok(false)
        }

        pub fn remove(&mut self, _: &str) -> Result<bool, EspError> {
            Ok(true)
        }
    }

    impl<T> embedded_svc::storage::StorageBase for EspNvs<T> {
        type Error = EspError;

        fn contains(&self, _: &str) -> Result<bool, Self::Error> {
            Ok(false)
        }

        fn remove(&mut self, _: &str) -> Result<bool, Self::Error> {
            Ok(true)
        }
    }

    impl<T> embedded_svc::storage::RawStorage for EspNvs<T> {
        fn len(&self, _: &str) -> Result<Option<usize>, Self::Error> {
            Ok(Some(0))
        }

        fn get_raw<'a>(&self, _: &str, _: &'a mut [u8]) -> Result<Option<&'a [u8]>, EspError> {
            Ok(None)
        }

        fn set_raw(&mut self, _: &str, _: &[u8]) -> Result<bool, Self::Error> {
            Ok(true)
        }
    }

    pub struct NvsDefault(());
    pub struct EspNvsPartition<T>(PhantomData<T>);
    pub type EspDefaultNvsPartition = EspNvsPartition<NvsDefault>;

    impl EspNvsPartition<NvsDefault> {
        pub fn take() -> Result<Self, EspError> {
            Ok(Self(PhantomData))
        }
    }
}

// vim: ts=4 sw=4 expandtab
