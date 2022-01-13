#[doc = "Register `AHBSTATUS` reader"]
pub struct R(crate::R<AHBSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NoPendingAhbReq` reader - If set, there's pending AHB Memory Read/Write request"]
pub struct NOPENDINGAHBREQ_R(crate::FieldReader<bool, bool>);
impl NOPENDINGAHBREQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NOPENDINGAHBREQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NOPENDINGAHBREQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AhbReadDataValid` reader - Set if AHB Read Data is Ready. This bit is Auto Clear Once a new AHB read Access kick off and Auto Set once AHB read data is valid."]
pub struct AHBREADDATAVALID_R(crate::FieldReader<bool, bool>);
impl AHBREADDATAVALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBREADDATAVALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBREADDATAVALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AhbReqFIFOFull` reader - Set if AHB Address/Data/Command FIFO is Full. (Read Only)"]
pub struct AHBREQFIFOFULL_R(crate::FieldReader<bool, bool>);
impl AHBREQFIFOFULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBREQFIFOFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBREQFIFOFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AhbReqFIFOhalfEmpty` reader - Set if AHB Address/Data/Command FIFO is less than half full."]
pub struct AHBREQFIFOHALFEMPTY_R(crate::FieldReader<bool, bool>);
impl AHBREQFIFOHALFEMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AHBREQFIFOHALFEMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBREQFIFOHALFEMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AhbReqFIFOSize` reader - 0 if FIFO is 4 entries, 1 if FIFO is 8 entries."]
pub struct AHBREQFIFOSIZE_R(crate::FieldReader<u8, u8>);
impl AHBREQFIFOSIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AHBREQFIFOSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AHBREQFIFOSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - If set, there's pending AHB Memory Read/Write request"]
    #[inline(always)]
    pub fn no_pending_ahb_req(&self) -> NOPENDINGAHBREQ_R {
        NOPENDINGAHBREQ_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set if AHB Read Data is Ready. This bit is Auto Clear Once a new AHB read Access kick off and Auto Set once AHB read data is valid."]
    #[inline(always)]
    pub fn ahb_read_data_valid(&self) -> AHBREADDATAVALID_R {
        AHBREADDATAVALID_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set if AHB Address/Data/Command FIFO is Full. (Read Only)"]
    #[inline(always)]
    pub fn ahb_req_fifofull(&self) -> AHBREQFIFOFULL_R {
        AHBREQFIFOFULL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set if AHB Address/Data/Command FIFO is less than half full."]
    #[inline(always)]
    pub fn ahb_req_fifohalf_empty(&self) -> AHBREQFIFOHALFEMPTY_R {
        AHBREQFIFOHALFEMPTY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - 0 if FIFO is 4 entries, 1 if FIFO is 8 entries."]
    #[inline(always)]
    pub fn ahb_req_fifosize(&self) -> AHBREQFIFOSIZE_R {
        AHBREQFIFOSIZE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
}
#[doc = "AHB status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbstatus](index.html) module"]
pub struct AHBSTATUS_SPEC;
impl crate::RegisterSpec for AHBSTATUS_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ahbstatus::R](R) reader structure"]
impl crate::Readable for AHBSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AHBSTATUS to value 0x08"]
impl crate::Resettable for AHBSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
