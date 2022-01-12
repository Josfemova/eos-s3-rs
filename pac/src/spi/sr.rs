#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BUSY` reader - SSI Busy Flag. When set, indicates that a serial transfer is in progress; when cleared indicates that the SPI Master is idle or disabled."]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFNF` reader - Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full."]
pub struct TFNF_R(crate::FieldReader<bool, bool>);
impl TFNF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFNF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFNF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Empty. When the transmit FIFO is completely empty, this bit is set. When the transmit FIFO contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt."]
pub struct TFE_R(crate::FieldReader<bool, bool>);
impl TFE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TFE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TFE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFNE` reader - Receive FIFO Not Empty. Set when the receive FIFO contains one or more entries and is cleared when the receive FIFO is empty. This bit can be polled by software to completely empty the receive FIFO."]
pub struct RFNE_R(crate::FieldReader<bool, bool>);
impl RFNE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFNE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFNE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RFF` reader - Receive FIFO Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared."]
pub struct RFF_R(crate::FieldReader<bool, bool>);
impl RFF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXE` reader - No function for SPI Master. Slave usage only."]
pub struct TXE_R(crate::FieldReader<bool, bool>);
impl TXE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TXE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DCOL` reader - Data Collision Error. Relevant only when the SPI Master is configured as a master device. This bit is set if the SPI Master is actively transmitting when another master selects this device as a slave. This informs the processor that the last data transfer was halted before completion. This bit is cleared when read."]
pub struct DCOL_R(crate::FieldReader<bool, bool>);
impl DCOL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DCOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DCOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - SSI Busy Flag. When set, indicates that a serial transfer is in progress; when cleared indicates that the SPI Master is idle or disabled."]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full."]
    #[inline(always)]
    pub fn tfnf(&self) -> TFNF_R {
        TFNF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO Empty. When the transmit FIFO is completely empty, this bit is set. When the transmit FIFO contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt."]
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Not Empty. Set when the receive FIFO contains one or more entries and is cleared when the receive FIFO is empty. This bit can be polled by software to completely empty the receive FIFO."]
    #[inline(always)]
    pub fn rfne(&self) -> RFNE_R {
        RFNE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Receive FIFO Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared."]
    #[inline(always)]
    pub fn rff(&self) -> RFF_R {
        RFF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - No function for SPI Master. Slave usage only."]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Data Collision Error. Relevant only when the SPI Master is configured as a master device. This bit is set if the SPI Master is actively transmitting when another master selects this device as a slave. This informs the processor that the last data transfer was halted before completion. This bit is cleared when read."]
    #[inline(always)]
    pub fn dcol(&self) -> DCOL_R {
        DCOL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "Status Register: This is a read-only register used to indicate the current transfer status, FIFO status, and any transmission/reception errors that may have occurred. This status register may be read at any time. None of the bits in this register request an interrupt.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
