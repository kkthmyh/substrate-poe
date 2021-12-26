use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};
use super::*;

// 创建存证测试
#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_ok!(PoeModule::create_claim(Origin::signed(1),claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), Some((1, frame_system::Pallet::<Test>::block_number())));
    })
}

// 创建存证错误场景-创建已存在的存证
#[test]
fn create_claim_failed_when_claim_already_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()) ,
            Error::<Test>::ProofAlreadyExist);
    })
}

// 创建存证错误场景-创建长度超过限制的存证
#[test]
fn create_claim_failed_when_length_exceeded() {
    new_test_ext().execute_with(|| {
        // 超出最大长度6的限制
        let claim = vec![0, 1, 2, 3, 4, 5, 6];
        // 符合要求的长度
        // let claim1 = vec![0, 1, 2, 3, 4];
        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()) ,
            Error::<Test>::ClaimLengthExceeded);
    })
}

// 撤销存证
#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&claim), None)
    })
}


// 撤销存折错误场景-撤销一个不存在的存证
#[test]
fn revoke_claim_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_noop!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()) ,
            Error::<Test>::ClaimNotExist);
    })
}

// 转移存证
#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        PoeModule::create_claim(Origin::signed(1), claim.clone());
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1),claim.clone(),2));
        assert_eq!(Proofs::<Test>::get(&claim), Some((2, frame_system::Pallet::<Test>::block_number())));
    })
}


// 转移错误-存证不存在场景
#[test]
fn transfer_claim_failed_when_claim_not_exist() {
    new_test_ext().execute_with(|| {
        let claim = vec![0, 1];
        assert_noop!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(),2),Error::<Test>::ClaimNotExist);
    })
}