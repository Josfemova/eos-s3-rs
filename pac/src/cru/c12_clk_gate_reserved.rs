#[doc = "Register `C12_CLK_GATE_Reserved` reader"]
pub struct R(crate::R<C12_CLK_GATE_RESERVED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C12_CLK_GATE_RESERVED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C12_CLK_GATE_RESERVED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C12_CLK_GATE_RESERVED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Gating control for clock 12\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [c12_clk_gate_reserved](index.html) module"]
pub struct C12_CLK_GATE_RESERVED_SPEC;
impl crate::RegisterSpec for C12_CLK_GATE_RESERVED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [c12_clk_gate_reserved::R](R) reader structure"]
impl crate::Readable for C12_CLK_GATE_RESERVED_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets C12_CLK_GATE_Reserved to value 0"]
impl crate::Resettable for C12_CLK_GATE_RESERVED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
