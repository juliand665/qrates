// Licensed under the MIT license <LICENSE or
// http://opensource.org/licenses/MIT>. This file may not be copied,
// modified, or distributed except according to those terms.

use corpus_database::types;
use rustc_hir as hir;
use rustc_middle::{mir, ty};

pub trait ConvertInto<T> {
    fn convert_into(&self) -> T;
}

impl ConvertInto<types::TyVisibility> for ty::Visibility<rustc_hir::def_id::DefId> {
    fn convert_into(&self) -> types::TyVisibility {
        match self {
            ty::Visibility::Public => types::TyVisibility::Public,
            ty::Visibility::Restricted(_) => types::TyVisibility::Restricted,
        }
    }
}

impl ConvertInto<types::Unsafety> for hir::Unsafety {
    fn convert_into(&self) -> types::Unsafety {
        match self {
            hir::Unsafety::Unsafe => types::Unsafety::Unsafe,
            hir::Unsafety::Normal => types::Unsafety::Normal,
        }
    }
}

impl ConvertInto<types::SpanExpansionKind> for rustc_span::hygiene::ExpnKind {
    fn convert_into(&self) -> types::SpanExpansionKind {
        use rustc_span::hygiene::AstPass;
        use rustc_span::hygiene::DesugaringKind;
        use rustc_span::hygiene::ExpnKind::*;
        use rustc_span::hygiene::MacroKind;
        match self {
            Root => types::SpanExpansionKind::Root,
            Macro(MacroKind::Bang, _) => types::SpanExpansionKind::MacroBang,
            Macro(MacroKind::Attr, _) => types::SpanExpansionKind::MacroAttr,
            Macro(MacroKind::Derive, _) => types::SpanExpansionKind::MacroDerive,
            AstPass(AstPass::StdImports) => types::SpanExpansionKind::AstPassStdImports,
            AstPass(AstPass::TestHarness) => types::SpanExpansionKind::AstPassTestHarness,
            AstPass(AstPass::ProcMacroHarness) => types::SpanExpansionKind::AstPassProcMacroHarness,
            Desugaring(DesugaringKind::CondTemporary) => {
                types::SpanExpansionKind::DesugaringCondTemporary
            }
            Desugaring(DesugaringKind::QuestionMark) => {
                types::SpanExpansionKind::DesugaringQuestionMark
            }
            Desugaring(DesugaringKind::TryBlock) => types::SpanExpansionKind::DesugaringTryBlock,
            Desugaring(DesugaringKind::OpaqueTy) => types::SpanExpansionKind::DesugaringOpaqueTy,
            Desugaring(DesugaringKind::Async) => types::SpanExpansionKind::DesugaringAsync,
            Desugaring(DesugaringKind::Await) => types::SpanExpansionKind::DesugaringAwait,
            Desugaring(DesugaringKind::ForLoop) => types::SpanExpansionKind::DesugaringForLoop,
            Desugaring(DesugaringKind::WhileLoop) => types::SpanExpansionKind::DesugaringWhileLoop,
            Desugaring(DesugaringKind::YeetExpr) => types::SpanExpansionKind::DesugaringYeetExpr,
            Inlined => types::SpanExpansionKind::Inlined,
        }
    }
}

impl ConvertInto<types::BlockCheckMode> for hir::BlockCheckMode {
    fn convert_into(&self) -> types::BlockCheckMode {
        use rustc_hir::BlockCheckMode::*;
        use rustc_hir::UnsafeSource::*;
        match self {
            DefaultBlock => types::BlockCheckMode::DefaultBlock,
            UnsafeBlock(CompilerGenerated) => types::BlockCheckMode::UnsafeBlockCompilerGenerated,
            UnsafeBlock(UserProvided) => types::BlockCheckMode::UnsafeBlockUserProvided,
        }
    }
}

impl ConvertInto<types::Mutability> for hir::Mutability {
    fn convert_into(&self) -> types::Mutability {
        match self {
            hir::Mutability::Mut => types::Mutability::Mutable,
            hir::Mutability::Not => types::Mutability::Immutable,
        }
    }
}

impl ConvertInto<types::Constness> for hir::Constness {
    fn convert_into(&self) -> types::Constness {
        match self {
            hir::Constness::Const => types::Constness::Const,
            hir::Constness::NotConst => types::Constness::NotConst,
        }
    }
}

impl ConvertInto<types::BorrowKind> for mir::BorrowKind {
    fn convert_into(&self) -> types::BorrowKind {
        match self {
            mir::BorrowKind::Shared => types::BorrowKind::Shared,
            mir::BorrowKind::Shallow => types::BorrowKind::Shallow,
            mir::BorrowKind::Unique => types::BorrowKind::Unique,
            mir::BorrowKind::Mut {
                allow_two_phase_borrow,
            } => {
                if *allow_two_phase_borrow {
                    types::BorrowKind::MutTwoPhase
                } else {
                    types::BorrowKind::Mut
                }
            }
        }
    }
}

impl ConvertInto<types::CastKind> for mir::CastKind {
    fn convert_into(&self) -> types::CastKind {
        match self {
            mir::CastKind::Pointer(pointer) => match pointer {
                ty::adjustment::PointerCast::ReifyFnPointer => types::CastKind::ReifyFnPointer,
                ty::adjustment::PointerCast::UnsafeFnPointer => types::CastKind::UnsafeFnPointer,
                ty::adjustment::PointerCast::ClosureFnPointer(usafety) => match usafety {
                    hir::Unsafety::Unsafe => types::CastKind::UnsafeClosureFnPointer,
                    hir::Unsafety::Normal => types::CastKind::NormalClosureFnPointer,
                },
                ty::adjustment::PointerCast::MutToConstPointer => {
                    types::CastKind::MutToConstPointer
                }
                ty::adjustment::PointerCast::ArrayToPointer => types::CastKind::ArrayToPointer,
                ty::adjustment::PointerCast::Unsize => types::CastKind::UnsizePointer,
            },
            mir::CastKind::PointerExposeAddress => types::CastKind::PointerExposeAddress,
            mir::CastKind::PointerFromExposedAddress => types::CastKind::PointerFromExposedAddress,
            mir::CastKind::DynStar => types::CastKind::DynStar,
            mir::CastKind::IntToInt => types::CastKind::IntToInt,
            mir::CastKind::FloatToInt => types::CastKind::FloatToInt,
            mir::CastKind::IntToFloat => types::CastKind::IntToFloat,
            mir::CastKind::FloatToFloat => types::CastKind::FloatToFloat,
            mir::CastKind::PtrToPtr => types::CastKind::PtrToPtr,
            mir::CastKind::FnPtrToPtr => types::CastKind::FnPtrToPtr,
        }
    }
}

impl<'tcx> ConvertInto<types::AggregateKind> for mir::AggregateKind<'tcx> {
    fn convert_into(&self) -> types::AggregateKind {
        match self {
            mir::AggregateKind::Array(..) => types::AggregateKind::Array,
            mir::AggregateKind::Tuple => types::AggregateKind::Tuple,
            mir::AggregateKind::Adt(..) => types::AggregateKind::Adt,
            mir::AggregateKind::Closure(..) => types::AggregateKind::Closure,
            mir::AggregateKind::Generator(..) => types::AggregateKind::Generator,
        }
    }
}

impl ConvertInto<types::ScopeSafety> for Option<mir::Safety> {
    fn convert_into(&self) -> types::ScopeSafety {
        match self {
            Some(mir::Safety::Safe) => types::ScopeSafety::Safe,
            Some(mir::Safety::BuiltinUnsafe) => types::ScopeSafety::BuiltinUnsafe,
            Some(mir::Safety::FnUnsafe) => types::ScopeSafety::FnUnsafe,
            Some(mir::Safety::ExplicitUnsafe(_)) => types::ScopeSafety::ExplicitUnsafe,
            None => types::ScopeSafety::Unknown,
        }
    }
}

impl ConvertInto<types::ImplPolarity> for hir::ImplPolarity {
    fn convert_into(&self) -> types::ImplPolarity {
        match self {
            hir::ImplPolarity::Positive => types::ImplPolarity::Positive,
            hir::ImplPolarity::Negative(_) => types::ImplPolarity::Negative,
        }
    }
}

impl<'tcx> ConvertInto<types::TyPrimitive> for ty::TyKind<'tcx> {
    fn convert_into(&self) -> types::TyPrimitive {
        use types::TyPrimitive::*;
        match self {
            ty::TyKind::Bool => Bool,
            ty::TyKind::Char => Char,
            ty::TyKind::Int(int_ty) => match int_ty {
                ty::IntTy::Isize => Isize,
                ty::IntTy::I8 => I8,
                ty::IntTy::I16 => I16,
                ty::IntTy::I32 => I32,
                ty::IntTy::I64 => I64,
                ty::IntTy::I128 => I128,
            },
            ty::TyKind::Uint(uint_ty) => match uint_ty {
                ty::UintTy::Usize => Usize,
                ty::UintTy::U8 => U8,
                ty::UintTy::U16 => U16,
                ty::UintTy::U32 => U32,
                ty::UintTy::U64 => U64,
                ty::UintTy::U128 => U128,
            },
            ty::TyKind::Float(float_ty) => match float_ty {
                ty::FloatTy::F32 => F32,
                ty::FloatTy::F64 => F64,
            },
            ty::TyKind::Str => Str,
            ty::TyKind::Never => Never,
            x => unreachable!("Not primitive type: {:?}", x),
        }
    }
}

impl ConvertInto<bool> for hir::IsAuto {
    fn convert_into(&self) -> bool {
        match self {
            hir::IsAuto::Yes => true,
            hir::IsAuto::No => false,
        }
    }
}

impl ConvertInto<types::Defaultness> for hir::Defaultness {
    fn convert_into(&self) -> types::Defaultness {
        match self {
            hir::Defaultness::Default { has_value } => {
                if *has_value {
                    types::Defaultness::DefaultWithValue
                } else {
                    types::Defaultness::DefaultNoValue
                }
            }
            hir::Defaultness::Final => types::Defaultness::Final,
        }
    }
}

impl ConvertInto<types::AdtKind> for ty::AdtKind {
    fn convert_into(&self) -> types::AdtKind {
        match self {
            ty::AdtKind::Struct => types::AdtKind::Struct,
            ty::AdtKind::Union => types::AdtKind::Union,
            ty::AdtKind::Enum => types::AdtKind::Enum,
        }
    }
}

impl ConvertInto<types::AdtVariantIndex> for rustc_target::abi::VariantIdx {
    fn convert_into(&self) -> types::AdtVariantIndex {
        self.index().into()
    }
}
