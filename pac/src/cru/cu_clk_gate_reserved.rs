#[doc = "Register `CU_CLK_GATE_Reserved` reader"]
pub struct R(crate::R<CU_CLK_GATE_RESERVED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CU_CLK_GATE_RESERVED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CU_CLK_GATE_RESERVED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CU_CLK_GATE_RESERVED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Not specified\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cu_clk_gate_reserved](index.html) module"]
pub struct CU_CLK_GATE_RESERVED_SPEC;
impl crate::RegisterSpec for CU_CLK_GATE_RESERVED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cu_clk_gate_reserved::R](R) reader structure"]
impl crate::Readable for CU_CLK_GATE_RESERVED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CU_CLK_GATE_Reserved to value 0"]
impl crate::Resettable for CU_CLK_GATE_RESERVED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
