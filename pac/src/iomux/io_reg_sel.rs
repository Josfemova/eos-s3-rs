#[doc = "Register `IO_REG_SEL` reader"]
pub struct R(crate::R<IO_REG_SEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_REG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_REG_SEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_REG_SEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO_REG_SEL` writer"]
pub struct W(crate::W<IO_REG_SEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO_REG_SEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IO_REG_SEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO_REG_SEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Select pad for IO maped to bit 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_SEL_0_A {
    #[doc = "0: Enable pad #6 as GPIO"]
    PAD_06 = 0,
    #[doc = "1: Enable pad #24 as GPIO"]
    PAD_24 = 1,
}
impl From<IO_SEL_0_A> for bool {
    #[inline(always)]
    fn from(variant: IO_SEL_0_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO_SEL_0` reader - Select pad for IO maped to bit 0"]
pub struct IO_SEL_0_R(crate::FieldReader<bool, IO_SEL_0_A>);
impl IO_SEL_0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_SEL_0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_SEL_0_A {
        match self.bits {
            false => IO_SEL_0_A::PAD_06,
            true => IO_SEL_0_A::PAD_24,
        }
    }
    #[doc = "Checks if the value of the field is `PAD_06`"]
    #[inline(always)]
    pub fn is_pad_06(&self) -> bool {
        **self == IO_SEL_0_A::PAD_06
    }
    #[doc = "Checks if the value of the field is `PAD_24`"]
    #[inline(always)]
    pub fn is_pad_24(&self) -> bool {
        **self == IO_SEL_0_A::PAD_24
    }
}
impl core::ops::Deref for IO_SEL_0_R {
    type Target = crate::FieldReader<bool, IO_SEL_0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_SEL_0` writer - Select pad for IO maped to bit 0"]
pub struct IO_SEL_0_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_SEL_0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_SEL_0_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable pad #6 as GPIO"]
    #[inline(always)]
    pub fn pad_06(self) -> &'a mut W {
        self.variant(IO_SEL_0_A::PAD_06)
    }
    #[doc = "Enable pad #24 as GPIO"]
    #[inline(always)]
    pub fn pad_24(self) -> &'a mut W {
        self.variant(IO_SEL_0_A::PAD_24)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Select pad for IO maped to bit 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_SEL_1_A {
    #[doc = "0: Enable pad #9 as GPIO"]
    PAD_09 = 0,
    #[doc = "1: Enable pad #26 as GPIO"]
    PAD_26 = 1,
}
impl From<IO_SEL_1_A> for bool {
    #[inline(always)]
    fn from(variant: IO_SEL_1_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO_SEL_1` reader - Select pad for IO maped to bit 1"]
pub struct IO_SEL_1_R(crate::FieldReader<bool, IO_SEL_1_A>);
impl IO_SEL_1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_SEL_1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_SEL_1_A {
        match self.bits {
            false => IO_SEL_1_A::PAD_09,
            true => IO_SEL_1_A::PAD_26,
        }
    }
    #[doc = "Checks if the value of the field is `PAD_09`"]
    #[inline(always)]
    pub fn is_pad_09(&self) -> bool {
        **self == IO_SEL_1_A::PAD_09
    }
    #[doc = "Checks if the value of the field is `PAD_26`"]
    #[inline(always)]
    pub fn is_pad_26(&self) -> bool {
        **self == IO_SEL_1_A::PAD_26
    }
}
impl core::ops::Deref for IO_SEL_1_R {
    type Target = crate::FieldReader<bool, IO_SEL_1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_SEL_1` writer - Select pad for IO maped to bit 1"]
pub struct IO_SEL_1_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_SEL_1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_SEL_1_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable pad #9 as GPIO"]
    #[inline(always)]
    pub fn pad_09(self) -> &'a mut W {
        self.variant(IO_SEL_1_A::PAD_09)
    }
    #[doc = "Enable pad #26 as GPIO"]
    #[inline(always)]
    pub fn pad_26(self) -> &'a mut W {
        self.variant(IO_SEL_1_A::PAD_26)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Select pad for IO maped to bit 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_SEL_2_A {
    #[doc = "0: Enable pad #11 as GPIO"]
    PAD_11 = 0,
    #[doc = "1: Enable pad #28 as GPIO"]
    PAD_28 = 1,
}
impl From<IO_SEL_2_A> for bool {
    #[inline(always)]
    fn from(variant: IO_SEL_2_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO_SEL_2` reader - Select pad for IO maped to bit 2"]
pub struct IO_SEL_2_R(crate::FieldReader<bool, IO_SEL_2_A>);
impl IO_SEL_2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_SEL_2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_SEL_2_A {
        match self.bits {
            false => IO_SEL_2_A::PAD_11,
            true => IO_SEL_2_A::PAD_28,
        }
    }
    #[doc = "Checks if the value of the field is `PAD_11`"]
    #[inline(always)]
    pub fn is_pad_11(&self) -> bool {
        **self == IO_SEL_2_A::PAD_11
    }
    #[doc = "Checks if the value of the field is `PAD_28`"]
    #[inline(always)]
    pub fn is_pad_28(&self) -> bool {
        **self == IO_SEL_2_A::PAD_28
    }
}
impl core::ops::Deref for IO_SEL_2_R {
    type Target = crate::FieldReader<bool, IO_SEL_2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_SEL_2` writer - Select pad for IO maped to bit 2"]
pub struct IO_SEL_2_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_SEL_2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_SEL_2_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable pad #11 as GPIO"]
    #[inline(always)]
    pub fn pad_11(self) -> &'a mut W {
        self.variant(IO_SEL_2_A::PAD_11)
    }
    #[doc = "Enable pad #28 as GPIO"]
    #[inline(always)]
    pub fn pad_28(self) -> &'a mut W {
        self.variant(IO_SEL_2_A::PAD_28)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Select pad for IO maped to bit 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_SEL_3_A {
    #[doc = "0: Enable pad #14 as GPIO"]
    PAD_14 = 0,
    #[doc = "1: Enable pad #30 as GPIO"]
    PAD_30 = 1,
}
impl From<IO_SEL_3_A> for bool {
    #[inline(always)]
    fn from(variant: IO_SEL_3_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO_SEL_3` reader - Select pad for IO maped to bit 3"]
pub struct IO_SEL_3_R(crate::FieldReader<bool, IO_SEL_3_A>);
impl IO_SEL_3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_SEL_3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_SEL_3_A {
        match self.bits {
            false => IO_SEL_3_A::PAD_14,
            true => IO_SEL_3_A::PAD_30,
        }
    }
    #[doc = "Checks if the value of the field is `PAD_14`"]
    #[inline(always)]
    pub fn is_pad_14(&self) -> bool {
        **self == IO_SEL_3_A::PAD_14
    }
    #[doc = "Checks if the value of the field is `PAD_30`"]
    #[inline(always)]
    pub fn is_pad_30(&self) -> bool {
        **self == IO_SEL_3_A::PAD_30
    }
}
impl core::ops::Deref for IO_SEL_3_R {
    type Target = crate::FieldReader<bool, IO_SEL_3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_SEL_3` writer - Select pad for IO maped to bit 3"]
pub struct IO_SEL_3_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_SEL_3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_SEL_3_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable pad #14 as GPIO"]
    #[inline(always)]
    pub fn pad_14(self) -> &'a mut W {
        self.variant(IO_SEL_3_A::PAD_14)
    }
    #[doc = "Enable pad #30 as GPIO"]
    #[inline(always)]
    pub fn pad_30(self) -> &'a mut W {
        self.variant(IO_SEL_3_A::PAD_30)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Select pad for IO maped to bit 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_SEL_4_A {
    #[doc = "0: Enable pad #18 as GPIO"]
    PAD_18 = 0,
    #[doc = "1: Enable pad #31 as GPIO"]
    PAD_31 = 1,
}
impl From<IO_SEL_4_A> for bool {
    #[inline(always)]
    fn from(variant: IO_SEL_4_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO_SEL_4` reader - Select pad for IO maped to bit 4"]
pub struct IO_SEL_4_R(crate::FieldReader<bool, IO_SEL_4_A>);
impl IO_SEL_4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_SEL_4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_SEL_4_A {
        match self.bits {
            false => IO_SEL_4_A::PAD_18,
            true => IO_SEL_4_A::PAD_31,
        }
    }
    #[doc = "Checks if the value of the field is `PAD_18`"]
    #[inline(always)]
    pub fn is_pad_18(&self) -> bool {
        **self == IO_SEL_4_A::PAD_18
    }
    #[doc = "Checks if the value of the field is `PAD_31`"]
    #[inline(always)]
    pub fn is_pad_31(&self) -> bool {
        **self == IO_SEL_4_A::PAD_31
    }
}
impl core::ops::Deref for IO_SEL_4_R {
    type Target = crate::FieldReader<bool, IO_SEL_4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_SEL_4` writer - Select pad for IO maped to bit 4"]
pub struct IO_SEL_4_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_SEL_4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_SEL_4_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable pad #18 as GPIO"]
    #[inline(always)]
    pub fn pad_18(self) -> &'a mut W {
        self.variant(IO_SEL_4_A::PAD_18)
    }
    #[doc = "Enable pad #31 as GPIO"]
    #[inline(always)]
    pub fn pad_31(self) -> &'a mut W {
        self.variant(IO_SEL_4_A::PAD_31)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Select pad for IO maped to bit 5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_SEL_5_A {
    #[doc = "0: Enable pad #21 as GPIO"]
    PAD_21 = 0,
    #[doc = "1: Enable pad #26 as GPIO"]
    PAD_26 = 1,
}
impl From<IO_SEL_5_A> for bool {
    #[inline(always)]
    fn from(variant: IO_SEL_5_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO_SEL_5` reader - Select pad for IO maped to bit 5"]
pub struct IO_SEL_5_R(crate::FieldReader<bool, IO_SEL_5_A>);
impl IO_SEL_5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_SEL_5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_SEL_5_A {
        match self.bits {
            false => IO_SEL_5_A::PAD_21,
            true => IO_SEL_5_A::PAD_26,
        }
    }
    #[doc = "Checks if the value of the field is `PAD_21`"]
    #[inline(always)]
    pub fn is_pad_21(&self) -> bool {
        **self == IO_SEL_5_A::PAD_21
    }
    #[doc = "Checks if the value of the field is `PAD_26`"]
    #[inline(always)]
    pub fn is_pad_26(&self) -> bool {
        **self == IO_SEL_5_A::PAD_26
    }
}
impl core::ops::Deref for IO_SEL_5_R {
    type Target = crate::FieldReader<bool, IO_SEL_5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_SEL_5` writer - Select pad for IO maped to bit 5"]
pub struct IO_SEL_5_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_SEL_5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_SEL_5_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable pad #21 as GPIO"]
    #[inline(always)]
    pub fn pad_21(self) -> &'a mut W {
        self.variant(IO_SEL_5_A::PAD_21)
    }
    #[doc = "Enable pad #26 as GPIO"]
    #[inline(always)]
    pub fn pad_26(self) -> &'a mut W {
        self.variant(IO_SEL_5_A::PAD_26)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Select pad for IO maped to bit 6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_SEL_6_A {
    #[doc = "0: Enable pad #22 as GPIO"]
    PAD_22 = 0,
    #[doc = "1: Enable pad #38 as GPIO"]
    PAD_38 = 1,
}
impl From<IO_SEL_6_A> for bool {
    #[inline(always)]
    fn from(variant: IO_SEL_6_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO_SEL_6` reader - Select pad for IO maped to bit 6"]
pub struct IO_SEL_6_R(crate::FieldReader<bool, IO_SEL_6_A>);
impl IO_SEL_6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_SEL_6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_SEL_6_A {
        match self.bits {
            false => IO_SEL_6_A::PAD_22,
            true => IO_SEL_6_A::PAD_38,
        }
    }
    #[doc = "Checks if the value of the field is `PAD_22`"]
    #[inline(always)]
    pub fn is_pad_22(&self) -> bool {
        **self == IO_SEL_6_A::PAD_22
    }
    #[doc = "Checks if the value of the field is `PAD_38`"]
    #[inline(always)]
    pub fn is_pad_38(&self) -> bool {
        **self == IO_SEL_6_A::PAD_38
    }
}
impl core::ops::Deref for IO_SEL_6_R {
    type Target = crate::FieldReader<bool, IO_SEL_6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_SEL_6` writer - Select pad for IO maped to bit 6"]
pub struct IO_SEL_6_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_SEL_6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_SEL_6_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable pad #22 as GPIO"]
    #[inline(always)]
    pub fn pad_22(self) -> &'a mut W {
        self.variant(IO_SEL_6_A::PAD_22)
    }
    #[doc = "Enable pad #38 as GPIO"]
    #[inline(always)]
    pub fn pad_38(self) -> &'a mut W {
        self.variant(IO_SEL_6_A::PAD_38)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Select pad for IO maped to bit 7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IO_SEL_7_A {
    #[doc = "0: Enable pad #23 as GPIO"]
    PAD_23 = 0,
    #[doc = "1: Enable pad #45 as GPIO"]
    PAD_45 = 1,
}
impl From<IO_SEL_7_A> for bool {
    #[inline(always)]
    fn from(variant: IO_SEL_7_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IO_SEL_7` reader - Select pad for IO maped to bit 7"]
pub struct IO_SEL_7_R(crate::FieldReader<bool, IO_SEL_7_A>);
impl IO_SEL_7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_SEL_7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IO_SEL_7_A {
        match self.bits {
            false => IO_SEL_7_A::PAD_23,
            true => IO_SEL_7_A::PAD_45,
        }
    }
    #[doc = "Checks if the value of the field is `PAD_23`"]
    #[inline(always)]
    pub fn is_pad_23(&self) -> bool {
        **self == IO_SEL_7_A::PAD_23
    }
    #[doc = "Checks if the value of the field is `PAD_45`"]
    #[inline(always)]
    pub fn is_pad_45(&self) -> bool {
        **self == IO_SEL_7_A::PAD_45
    }
}
impl core::ops::Deref for IO_SEL_7_R {
    type Target = crate::FieldReader<bool, IO_SEL_7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IO_SEL_7` writer - Select pad for IO maped to bit 7"]
pub struct IO_SEL_7_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_SEL_7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IO_SEL_7_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enable pad #23 as GPIO"]
    #[inline(always)]
    pub fn pad_23(self) -> &'a mut W {
        self.variant(IO_SEL_7_A::PAD_23)
    }
    #[doc = "Enable pad #45 as GPIO"]
    #[inline(always)]
    pub fn pad_45(self) -> &'a mut W {
        self.variant(IO_SEL_7_A::PAD_45)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Select pad for IO maped to bit 0"]
    #[inline(always)]
    pub fn io_sel_0(&self) -> IO_SEL_0_R {
        IO_SEL_0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Select pad for IO maped to bit 1"]
    #[inline(always)]
    pub fn io_sel_1(&self) -> IO_SEL_1_R {
        IO_SEL_1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select pad for IO maped to bit 2"]
    #[inline(always)]
    pub fn io_sel_2(&self) -> IO_SEL_2_R {
        IO_SEL_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Select pad for IO maped to bit 3"]
    #[inline(always)]
    pub fn io_sel_3(&self) -> IO_SEL_3_R {
        IO_SEL_3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Select pad for IO maped to bit 4"]
    #[inline(always)]
    pub fn io_sel_4(&self) -> IO_SEL_4_R {
        IO_SEL_4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Select pad for IO maped to bit 5"]
    #[inline(always)]
    pub fn io_sel_5(&self) -> IO_SEL_5_R {
        IO_SEL_5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Select pad for IO maped to bit 6"]
    #[inline(always)]
    pub fn io_sel_6(&self) -> IO_SEL_6_R {
        IO_SEL_6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Select pad for IO maped to bit 7"]
    #[inline(always)]
    pub fn io_sel_7(&self) -> IO_SEL_7_R {
        IO_SEL_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select pad for IO maped to bit 0"]
    #[inline(always)]
    pub fn io_sel_0(&mut self) -> IO_SEL_0_W {
        IO_SEL_0_W { w: self }
    }
    #[doc = "Bit 1 - Select pad for IO maped to bit 1"]
    #[inline(always)]
    pub fn io_sel_1(&mut self) -> IO_SEL_1_W {
        IO_SEL_1_W { w: self }
    }
    #[doc = "Bit 2 - Select pad for IO maped to bit 2"]
    #[inline(always)]
    pub fn io_sel_2(&mut self) -> IO_SEL_2_W {
        IO_SEL_2_W { w: self }
    }
    #[doc = "Bit 3 - Select pad for IO maped to bit 3"]
    #[inline(always)]
    pub fn io_sel_3(&mut self) -> IO_SEL_3_W {
        IO_SEL_3_W { w: self }
    }
    #[doc = "Bit 4 - Select pad for IO maped to bit 4"]
    #[inline(always)]
    pub fn io_sel_4(&mut self) -> IO_SEL_4_W {
        IO_SEL_4_W { w: self }
    }
    #[doc = "Bit 5 - Select pad for IO maped to bit 5"]
    #[inline(always)]
    pub fn io_sel_5(&mut self) -> IO_SEL_5_W {
        IO_SEL_5_W { w: self }
    }
    #[doc = "Bit 6 - Select pad for IO maped to bit 6"]
    #[inline(always)]
    pub fn io_sel_6(&mut self) -> IO_SEL_6_W {
        IO_SEL_6_W { w: self }
    }
    #[doc = "Bit 7 - Select pad for IO maped to bit 7"]
    #[inline(always)]
    pub fn io_sel_7(&mut self) -> IO_SEL_7_W {
        IO_SEL_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Selects which IO input will be registered (Pads used as GPIOS)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_reg_sel](index.html) module"]
pub struct IO_REG_SEL_SPEC;
impl crate::RegisterSpec for IO_REG_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io_reg_sel::R](R) reader structure"]
impl crate::Readable for IO_REG_SEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io_reg_sel::W](W) writer structure"]
impl crate::Writable for IO_REG_SEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IO_REG_SEL to value 0"]
impl crate::Resettable for IO_REG_SEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
