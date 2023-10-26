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

pub mod netif {
    use dummy_esp_idf_sys::EspError;
    use embedded_svc::ipv4::IpInfo;

    pub struct EspNetif(pub ());

    impl EspNetif {
        pub fn get_ip_info(&self) -> Result<IpInfo, EspError> {
            Err(EspError::from(1).unwrap())
        }
    }
}

pub mod wifi {
    use super::eventloop::EspSystemEventLoop;
    use super::netif::EspNetif;
    use super::nvs::EspDefaultNvsPartition;
    use dummy_esp_idf_hal::{modem::WifiModemPeripheral, peripheral::Peripheral};
    use dummy_esp_idf_sys::EspError;
    use embedded_svc::wifi::{AccessPointInfo, Configuration};
    use std::marker::PhantomData;

    pub mod config {
        #[derive(Clone, Debug, PartialEq, Eq, Default)]
        pub struct ScanConfig {
            pub bssid: Option<[u8; 6]>,
            pub ssid: Option<heapless::String<32>>,
            pub channel: Option<u8>,
            pub scan_type: ScanType,
            pub show_hidden: bool,
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum ScanType {
            Active {
                min: core::time::Duration,
                max: core::time::Duration,
            },
            Passive(core::time::Duration),
        }
        impl Default for ScanType {
            fn default() -> Self {
                ScanType::Active {
                    min: core::time::Duration::from_secs(0),
                    max: core::time::Duration::from_secs(0),
                }
            }
        }
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum WifiDeviceId {
        Ap,
        Sta,
    }

    pub struct EspWifi<'a> {
        drv: WifiDriver<'a>,
        sta_netif: EspNetif,
    }

    impl<'a> EspWifi<'a> {
        pub fn new<M: WifiModemPeripheral>(
            modem: impl Peripheral<P = M> + 'a,
            sysloop: EspSystemEventLoop,
            nvs: Option<EspDefaultNvsPartition>,
        ) -> Result<Self, EspError> {
            Ok(Self {
                drv: WifiDriver::new(modem, sysloop, nvs).unwrap(),
                sta_netif: EspNetif(()),
            })
        }

        pub fn driver(&self) -> &WifiDriver<'a> {
            &self.drv
        }

        pub fn driver_mut(&mut self) -> &mut WifiDriver<'a> {
            &mut self.drv
        }

        pub fn start(&mut self) -> Result<(), EspError> {
            Ok(())
        }

        pub fn stop(&mut self) -> Result<(), EspError> {
            Ok(())
        }

        pub fn connect(&mut self) -> Result<(), EspError> {
            Ok(())
        }

        pub fn disconnect(&mut self) -> Result<(), EspError> {
            Ok(())
        }

        pub fn set_configuration(&mut self, _: &Configuration) -> Result<(), EspError> {
            Ok(())
        }

        pub fn start_scan(&mut self, _: &config::ScanConfig, _: bool) -> Result<(), EspError> {
            Ok(())
        }

        pub fn stop_scan(&mut self) -> Result<(), EspError> {
            Ok(())
        }

        pub fn is_scan_done(&self) -> Result<bool, EspError> {
            Ok(true)
        }

        pub fn get_scan_result(&mut self) -> Result<Vec<AccessPointInfo>, EspError> {
            Ok(vec![])
        }

        pub fn sta_netif(&self) -> &EspNetif {
            &self.sta_netif
        }
    }

    pub struct WifiDriver<'a>(PhantomData<&'a ()>);

    impl<'a> WifiDriver<'a> {
        pub fn new<M: WifiModemPeripheral>(
            _: impl Peripheral<P = M> + 'a,
            _: EspSystemEventLoop,
            _: Option<EspDefaultNvsPartition>,
        ) -> Result<Self, EspError> {
            Ok(Self(PhantomData))
        }

        pub fn set_callbacks<R, T>(
            &mut self,
            _rx_callback: R,
            _tx_callback: T,
        ) -> Result<(), EspError>
        where
            R: FnMut(WifiDeviceId, &[u8]) -> Result<(), EspError> + Send + 'static,
            T: FnMut(WifiDeviceId, &[u8], bool) + Send + 'static,
        {
            Ok(())
        }

        pub fn is_connected(&self) -> Result<bool, EspError> {
            Ok(true)
        }

        pub fn is_sta_connected(&self) -> Result<bool, EspError> {
            Ok(true)
        }

        pub fn is_started(&self) -> Result<bool, EspError> {
            Ok(true)
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

        pub fn get_raw<'a>(&self, _: &str, _: &'a mut [u8]) -> Result<Option<&'a [u8]>, EspError> {
            Ok(Some(&[]))
        }

        pub fn set_raw(&mut self, _: &str, _: &[u8]) -> Result<bool, EspError> {
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
