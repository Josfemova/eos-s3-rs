#[doc = "Register `UART_MIS` reader"]
pub struct R(crate::R<UART_MIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_MIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_MIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_MIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RIMMIS` reader - nUARTRI modem masked interrupt status (masked interrupt state)"]
pub struct RIMMIS_R(crate::FieldReader<bool, bool>);
impl RIMMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSMMIS` reader - nUARTCTS modem masked interrupt status (masked interrupt state)"]
pub struct CTSMMIS_R(crate::FieldReader<bool, bool>);
impl CTSMMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDMMIS` reader - nUARTDCD modem masked interrupt status (masked interrupt state)"]
pub struct DCDMMIS_R(crate::FieldReader<bool, bool>);
impl DCDMMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRMMIS` reader - nUARTDSR modem masked interrupt status (masked interrupt state)"]
pub struct DSRMMIS_R(crate::FieldReader<bool, bool>);
impl DSRMMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSRMMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRMMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXMIS` reader - Receive masked interrupt status (masked interrupt state)"]
pub struct RXMIS_R(crate::FieldReader<bool, bool>);
impl RXMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXMIS` reader - Transmit masked interrupt status (masked interrupt state)"]
pub struct TXMIS_R(crate::FieldReader<bool, bool>);
impl TXMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTMIS` reader - Receive timeout masked interrupt status (masked interrupt state)"]
pub struct RTMIS_R(crate::FieldReader<bool, bool>);
impl RTMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEMIS` reader - Framing error masked interrupt status (masked interrupt state)"]
pub struct FEMIS_R(crate::FieldReader<bool, bool>);
impl FEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEMIS` reader - Parity error masked interrupt status (masked interrupt state)"]
pub struct PEMIS_R(crate::FieldReader<bool, bool>);
impl PEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEMIS` reader - Break error masked interrupt status (masked interrupt state)"]
pub struct BEMIS_R(crate::FieldReader<bool, bool>);
impl BEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OEMIS` reader - Overrun error masked interrupt status (masked interrupt state)"]
pub struct OEMIS_R(crate::FieldReader<bool, bool>);
impl OEMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OEMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OEMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - nUARTRI modem masked interrupt status (masked interrupt state)"]
    #[inline(always)]
    pub fn rimmis(&self) -> RIMMIS_R {
        RIMMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS modem masked interrupt status (masked interrupt state)"]
    #[inline(always)]
    pub fn ctsmmis(&self) -> CTSMMIS_R {
        CTSMMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD modem masked interrupt status (masked interrupt state)"]
    #[inline(always)]
    pub fn dcdmmis(&self) -> DCDMMIS_R {
        DCDMMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR modem masked interrupt status (masked interrupt state)"]
    #[inline(always)]
    pub fn dsrmmis(&self) -> DSRMMIS_R {
        DSRMMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive masked interrupt status (masked interrupt state)"]
    #[inline(always)]
    pub fn rxmis(&self) -> RXMIS_R {
        RXMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit masked interrupt status (masked interrupt state)"]
    #[inline(always)]
    pub fn txmis(&self) -> TXMIS_R {
        TXMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive timeout masked interrupt status (masked interrupt state)"]
    #[inline(always)]
    pub fn rtmis(&self) -> RTMIS_R {
        RTMIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Framing error masked interrupt status (masked interrupt state)"]
    #[inline(always)]
    pub fn femis(&self) -> FEMIS_R {
        FEMIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity error masked interrupt status (masked interrupt state)"]
    #[inline(always)]
    pub fn pemis(&self) -> PEMIS_R {
        PEMIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Break error masked interrupt status (masked interrupt state)"]
    #[inline(always)]
    pub fn bemis(&self) -> BEMIS_R {
        BEMIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Overrun error masked interrupt status (masked interrupt state)"]
    #[inline(always)]
    pub fn oemis(&self) -> OEMIS_R {
        OEMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "Masked interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_mis](index.html) module"]
pub struct UART_MIS_SPEC;
impl crate::RegisterSpec for UART_MIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_mis::R](R) reader structure"]
impl crate::Readable for UART_MIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_MIS to value 0"]
impl crate::Resettable for UART_MIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
