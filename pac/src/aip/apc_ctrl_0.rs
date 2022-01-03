#[doc = "Register `APC_CTRL_0` reader"]
pub struct R(crate::R<APC_CTRL_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APC_CTRL_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APC_CTRL_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APC_CTRL_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `dis` reader - 1'b0 : APC ON, Always ON"]
pub struct DIS_R(crate::FieldReader<bool, bool>);
impl DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - 1'b0 : APC ON, Always ON"]
    #[inline(always)]
    pub fn dis(&self) -> DIS_R {
        DIS_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "APC control register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apc_ctrl_0](index.html) module"]
pub struct APC_CTRL_0_SPEC;
impl crate::RegisterSpec for APC_CTRL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apc_ctrl_0::R](R) reader structure"]
impl crate::Readable for APC_CTRL_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APC_CTRL_0 to value 0"]
impl crate::Resettable for APC_CTRL_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
