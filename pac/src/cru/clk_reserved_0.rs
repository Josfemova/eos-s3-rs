#[doc = "Register `CLK_RESERVED_0` reader"]
pub struct R(crate::R<CLK_RESERVED_0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_RESERVED_0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_RESERVED_0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_RESERVED_0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Not specified\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_reserved_0](index.html) module"]
pub struct CLK_RESERVED_0_SPEC;
impl crate::RegisterSpec for CLK_RESERVED_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_reserved_0::R](R) reader structure"]
impl crate::Readable for CLK_RESERVED_0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLK_RESERVED_0 to value 0"]
impl crate::Resettable for CLK_RESERVED_0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
