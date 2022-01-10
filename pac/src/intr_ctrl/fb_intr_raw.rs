#[doc = "Register `FB_INTR_RAW` reader"]
pub struct R(crate::R<FB_INTR_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FB_INTR_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FB_INTR_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FB_INTR_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FB_0_INTR_RAW` reader - Raw interrupt for Fabric. This register will reflect the value of the Fabric regardless of the type/polarity"]
pub struct FB_0_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl FB_0_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_0_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_0_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_1_INTR_RAW` reader - Raw interrupt for Fabric. This register will reflect the value of the Fabric regardless of the type/polarity"]
pub struct FB_1_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl FB_1_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_1_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_1_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_2_INTR_RAW` reader - Raw interrupt for Fabric. This register will reflect the value of the Fabric regardless of the type/polarity"]
pub struct FB_2_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl FB_2_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_2_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_2_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FB_3_INTR_RAW` reader - Raw interrupt for Fabric. This register will reflect the value of the Fabric regardless of the type/polarity"]
pub struct FB_3_INTR_RAW_R(crate::FieldReader<bool, bool>);
impl FB_3_INTR_RAW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FB_3_INTR_RAW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FB_3_INTR_RAW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Raw interrupt for Fabric. This register will reflect the value of the Fabric regardless of the type/polarity"]
    #[inline(always)]
    pub fn fb_0_intr_raw(&self) -> FB_0_INTR_RAW_R {
        FB_0_INTR_RAW_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Raw interrupt for Fabric. This register will reflect the value of the Fabric regardless of the type/polarity"]
    #[inline(always)]
    pub fn fb_1_intr_raw(&self) -> FB_1_INTR_RAW_R {
        FB_1_INTR_RAW_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Raw interrupt for Fabric. This register will reflect the value of the Fabric regardless of the type/polarity"]
    #[inline(always)]
    pub fn fb_2_intr_raw(&self) -> FB_2_INTR_RAW_R {
        FB_2_INTR_RAW_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Raw interrupt for Fabric. This register will reflect the value of the Fabric regardless of the type/polarity"]
    #[inline(always)]
    pub fn fb_3_intr_raw(&self) -> FB_3_INTR_RAW_R {
        FB_3_INTR_RAW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
#[doc = "FB raw interrupt indicators\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fb_intr_raw](index.html) module"]
pub struct FB_INTR_RAW_SPEC;
impl crate::RegisterSpec for FB_INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fb_intr_raw::R](R) reader structure"]
impl crate::Readable for FB_INTR_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FB_INTR_RAW to value 0"]
impl crate::Resettable for FB_INTR_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
