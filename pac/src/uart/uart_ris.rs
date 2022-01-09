#[doc = "Register `UART_RIS` reader"]
pub struct R(crate::R<UART_RIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART_RIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART_RIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART_RIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RIRMIS` reader - nUARTRI interrupt status (raw interrupt state)"]
pub struct RIRMIS_R(crate::FieldReader<bool, bool>);
impl RIRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RIRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RIRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTSRMIS` reader - nUARTCTS interrupt status (raw interrupt state)"]
pub struct CTSRMIS_R(crate::FieldReader<bool, bool>);
impl CTSRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTSRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTSRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCDRMIS` reader - nUARTDCD interrupt status (raw interrupt state)"]
pub struct DCDRMIS_R(crate::FieldReader<bool, bool>);
impl DCDRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCDRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCDRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSRRMIS` reader - nUARTDSR interrupt status (raw interrupt state)"]
pub struct DSRRMIS_R(crate::FieldReader<bool, bool>);
impl DSRRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DSRRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSRRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXRMIS` reader - Receive interrupt status (raw interrupt state)"]
pub struct RXRMIS_R(crate::FieldReader<bool, bool>);
impl RXRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RXRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXRMIS` reader - Transmit interrupt status (raw interrupt state)"]
pub struct TXRMIS_R(crate::FieldReader<bool, bool>);
impl TXRMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXRMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXRMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTRIS` reader - Receive timeout interrupt status (raw interrupt state)"]
pub struct RTRIS_R(crate::FieldReader<bool, bool>);
impl RTRIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTRIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTRIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FEIS` reader - Framing error interrupt status (raw interrupt state)"]
pub struct FEIS_R(crate::FieldReader<bool, bool>);
impl FEIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FEIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEIS` reader - Parity error interrupt status (raw interrupt state)"]
pub struct PEIS_R(crate::FieldReader<bool, bool>);
impl PEIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEIS` reader - Break error interrupt status (raw interrupt state)"]
pub struct BEIS_R(crate::FieldReader<bool, bool>);
impl BEIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BEIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OERMIS` reader - Overrun error interrupt status (raw interrupt state)"]
pub struct OERMIS_R(crate::FieldReader<bool, bool>);
impl OERMIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OERMIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OERMIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - nUARTRI interrupt status (raw interrupt state)"]
    #[inline(always)]
    pub fn rirmis(&self) -> RIRMIS_R {
        RIRMIS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - nUARTCTS interrupt status (raw interrupt state)"]
    #[inline(always)]
    pub fn ctsrmis(&self) -> CTSRMIS_R {
        CTSRMIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - nUARTDCD interrupt status (raw interrupt state)"]
    #[inline(always)]
    pub fn dcdrmis(&self) -> DCDRMIS_R {
        DCDRMIS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - nUARTDSR interrupt status (raw interrupt state)"]
    #[inline(always)]
    pub fn dsrrmis(&self) -> DSRRMIS_R {
        DSRRMIS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive interrupt status (raw interrupt state)"]
    #[inline(always)]
    pub fn rxrmis(&self) -> RXRMIS_R {
        RXRMIS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Transmit interrupt status (raw interrupt state)"]
    #[inline(always)]
    pub fn txrmis(&self) -> TXRMIS_R {
        TXRMIS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Receive timeout interrupt status (raw interrupt state)"]
    #[inline(always)]
    pub fn rtris(&self) -> RTRIS_R {
        RTRIS_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Framing error interrupt status (raw interrupt state)"]
    #[inline(always)]
    pub fn feis(&self) -> FEIS_R {
        FEIS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Parity error interrupt status (raw interrupt state)"]
    #[inline(always)]
    pub fn peis(&self) -> PEIS_R {
        PEIS_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Break error interrupt status (raw interrupt state)"]
    #[inline(always)]
    pub fn beis(&self) -> BEIS_R {
        BEIS_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Overrun error interrupt status (raw interrupt state)"]
    #[inline(always)]
    pub fn oermis(&self) -> OERMIS_R {
        OERMIS_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
#[doc = "Raw interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart_ris](index.html) module"]
pub struct UART_RIS_SPEC;
impl crate::RegisterSpec for UART_RIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart_ris::R](R) reader structure"]
impl crate::Readable for UART_RIS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UART_RIS to value 0"]
impl crate::Resettable for UART_RIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
