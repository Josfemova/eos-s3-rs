#[doc = "Register `M4_SRAM_STATUS` reader"]
pub struct R(crate::R<M4_SRAM_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<M4_SRAM_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<M4_SRAM_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<M4_SRAM_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `M4S0` reader - Bit is set if M4S0 SRAM power domain is on. Same as M4S0_SRAM_STATUS\\[0:0\\]"]
pub struct M4S0_R(crate::FieldReader<bool, bool>);
impl M4S0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S1` reader - Bit is set if M4S1 SRAM power domain is on"]
pub struct M4S1_R(crate::FieldReader<bool, bool>);
impl M4S1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S2` reader - Bit is set if M4S2 SRAM power domain is on"]
pub struct M4S2_R(crate::FieldReader<bool, bool>);
impl M4S2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S3` reader - Bit is set if M4S3 SRAM power domain is on"]
pub struct M4S3_R(crate::FieldReader<bool, bool>);
impl M4S3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S4` reader - Bit is set if M4S4 SRAM power domain is on"]
pub struct M4S4_R(crate::FieldReader<bool, bool>);
impl M4S4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S5` reader - Bit is set if M4S5 SRAM power domain is on"]
pub struct M4S5_R(crate::FieldReader<bool, bool>);
impl M4S5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S6` reader - Bit is set if M4S6 SRAM power domain is on"]
pub struct M4S6_R(crate::FieldReader<bool, bool>);
impl M4S6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S7` reader - Bit is set if M4S7 SRAM power domain is on"]
pub struct M4S7_R(crate::FieldReader<bool, bool>);
impl M4S7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S8` reader - Bit is set if M4S8 SRAM power domain is on"]
pub struct M4S8_R(crate::FieldReader<bool, bool>);
impl M4S8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S9` reader - Bit is set if M4S9 SRAM power domain is on"]
pub struct M4S9_R(crate::FieldReader<bool, bool>);
impl M4S9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S10` reader - Bit is set if M4S10 SRAM power domain is on"]
pub struct M4S10_R(crate::FieldReader<bool, bool>);
impl M4S10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S11` reader - Bit is set if M4S11 SRAM power domain is on"]
pub struct M4S11_R(crate::FieldReader<bool, bool>);
impl M4S11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S12` reader - Bit is set if M4S12 SRAM power domain is on"]
pub struct M4S12_R(crate::FieldReader<bool, bool>);
impl M4S12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S13` reader - Bit is set if M4S13 SRAM power domain is on"]
pub struct M4S13_R(crate::FieldReader<bool, bool>);
impl M4S13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S14` reader - Bit is set if M4S14 SRAM power domain is on"]
pub struct M4S14_R(crate::FieldReader<bool, bool>);
impl M4S14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `M4S15` reader - Bit is set if M4S15 SRAM power domain is on"]
pub struct M4S15_R(crate::FieldReader<bool, bool>);
impl M4S15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        M4S15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for M4S15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Bit is set if M4S0 SRAM power domain is on. Same as M4S0_SRAM_STATUS\\[0:0\\]"]
    #[inline(always)]
    pub fn m4s0(&self) -> M4S0_R {
        M4S0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bit is set if M4S1 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s1(&self) -> M4S1_R {
        M4S1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bit is set if M4S2 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s2(&self) -> M4S2_R {
        M4S2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bit is set if M4S3 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s3(&self) -> M4S3_R {
        M4S3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Bit is set if M4S4 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s4(&self) -> M4S4_R {
        M4S4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Bit is set if M4S5 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s5(&self) -> M4S5_R {
        M4S5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Bit is set if M4S6 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s6(&self) -> M4S6_R {
        M4S6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Bit is set if M4S7 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s7(&self) -> M4S7_R {
        M4S7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Bit is set if M4S8 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s8(&self) -> M4S8_R {
        M4S8_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Bit is set if M4S9 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s9(&self) -> M4S9_R {
        M4S9_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Bit is set if M4S10 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s10(&self) -> M4S10_R {
        M4S10_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Bit is set if M4S11 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s11(&self) -> M4S11_R {
        M4S11_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Bit is set if M4S12 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s12(&self) -> M4S12_R {
        M4S12_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Bit is set if M4S13 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s13(&self) -> M4S13_R {
        M4S13_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Bit is set if M4S14 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s14(&self) -> M4S14_R {
        M4S14_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Bit is set if M4S15 SRAM power domain is on"]
    #[inline(always)]
    pub fn m4s15(&self) -> M4S15_R {
        M4S15_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "M4 SRAM Power domain status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [m4_sram_status](index.html) module"]
pub struct M4_SRAM_STATUS_SPEC;
impl crate::RegisterSpec for M4_SRAM_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [m4_sram_status::R](R) reader structure"]
impl crate::Readable for M4_SRAM_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets M4_SRAM_STATUS to value 0"]
impl crate::Resettable for M4_SRAM_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
