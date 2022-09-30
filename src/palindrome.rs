/*
fn check_palindrome(palavra: &[u8; size], index: usize) -> u8 {
    let mut i = index;
    if palavra[i] % 2 != 0 {
        loop {
            if i < (size - 1) - i {
                println!("{}", i);
                if palavra[i] != palavra[(size - 1) - i] {
                    check_palindrome(palavra, index + 1);
                    println!("false {}", i);
                    return index;
                }
            } else {
                println!("false {}", i);
                return 0;
            }
            i = i + 1;
        }
    }
    println!("false {}", i);
}
*/

pub fn check_palindrome(palavra: &[u8; 40]) -> u64 {
    if palavra[0] % 2 != 0 {
        if palavra[0] == palavra[36] {
            if palavra[1] == palavra[35] {
                if palavra[2] == palavra[34] {
                    if palavra[3] == palavra[33] {
                        if palavra[4] == palavra[32] {
                            if palavra[5] == palavra[31] {
                                if palavra[6] == palavra[30] {
                                    if palavra[7] == palavra[29] {
                                        if palavra[8] == palavra[28] {
                                            if palavra[9] == palavra[27] {
                                                if palavra[10] == palavra[26] {
                                                    if palavra[11] == palavra[25] {
                                                        if palavra[12] == palavra[24] {
                                                            if palavra[13] == palavra[23] {
                                                                if palavra[14] == palavra[22] {
                                                                    if palavra[15] == palavra[21] {
                                                                        if palavra[16]
                                                                            == palavra[20]
                                                                        {
                                                                            if palavra[17]
                                                                                == palavra[19]
                                                                            {
                                                                                return 0;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[2] == palavra[36] {
            if palavra[3] == palavra[35] {
                if palavra[4] == palavra[34] {
                    if palavra[5] == palavra[33] {
                        if palavra[6] == palavra[32] {
                            if palavra[7] == palavra[31] {
                                if palavra[8] == palavra[30] {
                                    if palavra[9] == palavra[29] {
                                        if palavra[10] == palavra[28] {
                                            if palavra[11] == palavra[27] {
                                                if palavra[12] == palavra[26] {
                                                    if palavra[13] == palavra[25] {
                                                        if palavra[14] == palavra[24] {
                                                            if palavra[15] == palavra[23] {
                                                                if palavra[16] == palavra[22] {
                                                                    if palavra[17] == palavra[21] {
                                                                        if palavra[18]
                                                                            == palavra[20]
                                                                        {
                                                                            if palavra[19]
                                                                                == palavra[19]
                                                                            {
                                                                                return 1 * 2;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[4] == palavra[36] {
            if palavra[5] == palavra[35] {
                if palavra[6] == palavra[34] {
                    if palavra[7] == palavra[33] {
                        if palavra[8] == palavra[32] {
                            if palavra[9] == palavra[31] {
                                if palavra[10] == palavra[30] {
                                    if palavra[11] == palavra[29] {
                                        if palavra[12] == palavra[28] {
                                            if palavra[13] == palavra[27] {
                                                if palavra[14] == palavra[26] {
                                                    if palavra[15] == palavra[25] {
                                                        if palavra[16] == palavra[24] {
                                                            if palavra[17] == palavra[23] {
                                                                if palavra[18] == palavra[22] {
                                                                    if palavra[19] == palavra[21] {
                                                                        if palavra[21]
                                                                            == palavra[19]
                                                                        {
                                                                            return 2 * 2;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[6] == palavra[36] {
            if palavra[7] == palavra[35] {
                if palavra[8] == palavra[34] {
                    if palavra[9] == palavra[33] {
                        if palavra[10] == palavra[32] {
                            if palavra[11] == palavra[31] {
                                if palavra[12] == palavra[30] {
                                    if palavra[13] == palavra[29] {
                                        if palavra[14] == palavra[28] {
                                            if palavra[15] == palavra[27] {
                                                if palavra[16] == palavra[26] {
                                                    if palavra[17] == palavra[25] {
                                                        if palavra[18] == palavra[24] {
                                                            if palavra[19] == palavra[23] {
                                                                if palavra[20] == palavra[22] {
                                                                    return 3 * 2;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[8] == palavra[36] {
            if palavra[9] == palavra[35] {
                if palavra[10] == palavra[34] {
                    if palavra[11] == palavra[33] {
                        if palavra[12] == palavra[32] {
                            if palavra[13] == palavra[31] {
                                if palavra[14] == palavra[30] {
                                    if palavra[15] == palavra[29] {
                                        if palavra[16] == palavra[28] {
                                            if palavra[17] == palavra[27] {
                                                if palavra[18] == palavra[26] {
                                                    if palavra[19] == palavra[25] {
                                                        if palavra[20] == palavra[24] {
                                                            if palavra[21] == palavra[23] {
                                                                return 4 * 2;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[10] == palavra[36] {
            if palavra[11] == palavra[35] {
                if palavra[12] == palavra[34] {
                    if palavra[13] == palavra[33] {
                        if palavra[14] == palavra[32] {
                            if palavra[15] == palavra[31] {
                                if palavra[16] == palavra[30] {
                                    if palavra[17] == palavra[29] {
                                        if palavra[18] == palavra[28] {
                                            if palavra[19] == palavra[27] {
                                                if palavra[20] == palavra[26] {
                                                    if palavra[21] == palavra[25] {
                                                        if palavra[22] == palavra[24] {
                                                            return 5 * 2;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[12] == palavra[36] {
            if palavra[13] == palavra[35] {
                if palavra[14] == palavra[34] {
                    if palavra[15] == palavra[33] {
                        if palavra[16] == palavra[32] {
                            if palavra[17] == palavra[31] {
                                if palavra[18] == palavra[30] {
                                    if palavra[19] == palavra[29] {
                                        if palavra[20] == palavra[28] {
                                            if palavra[21] == palavra[27] {
                                                if palavra[22] == palavra[26] {
                                                    if palavra[23] == palavra[25] {
                                                        return 6 * 2;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[14] == palavra[36] {
            if palavra[15] == palavra[35] {
                if palavra[16] == palavra[34] {
                    if palavra[17] == palavra[33] {
                        if palavra[18] == palavra[32] {
                            if palavra[19] == palavra[31] {
                                if palavra[20] == palavra[30] {
                                    if palavra[21] == palavra[29] {
                                        if palavra[22] == palavra[28] {
                                            if palavra[23] == palavra[27] {
                                                if palavra[24] == palavra[26] {
                                                    return 7 * 2;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[16] == palavra[36] {
            if palavra[17] == palavra[35] {
                if palavra[18] == palavra[34] {
                    if palavra[19] == palavra[33] {
                        if palavra[20] == palavra[32] {
                            if palavra[21] == palavra[31] {
                                if palavra[22] == palavra[30] {
                                    if palavra[23] == palavra[29] {
                                        if palavra[24] == palavra[28] {
                                            if palavra[25] == palavra[27] {
                                                return 8 * 2;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[18] == palavra[36] {
            if palavra[19] == palavra[35] {
                if palavra[20] == palavra[34] {
                    if palavra[21] == palavra[33] {
                        if palavra[22] == palavra[32] {
                            if palavra[23] == palavra[31] {
                                if palavra[24] == palavra[30] {
                                    if palavra[25] == palavra[29] {
                                        if palavra[26] == palavra[28] {
                                            return 9 * 2;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[20] == palavra[36] {
            if palavra[21] == palavra[35] {
                if palavra[22] == palavra[34] {
                    if palavra[23] == palavra[33] {
                        if palavra[24] == palavra[32] {
                            if palavra[25] == palavra[31] {
                                if palavra[26] == palavra[30] {
                                    if palavra[27] == palavra[29] {
                                        return 10 * 2;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[22] == palavra[36] {
            if palavra[23] == palavra[35] {
                if palavra[24] == palavra[34] {
                    if palavra[25] == palavra[33] {
                        if palavra[26] == palavra[32] {
                            if palavra[27] == palavra[31] {
                                if palavra[28] == palavra[30] {
                                    return 11 * 2;
                                }
                            }
                        }
                    }
                }
            }
        } else if palavra[24] == palavra[36] {
            if palavra[25] == palavra[35] {
                if palavra[26] == palavra[34] {
                    if palavra[27] == palavra[33] {
                        if palavra[28] == palavra[32] {
                            if palavra[29] == palavra[31] {
                                return 12 * 2;
                            }
                        }
                    }
                }
            }
        } else if palavra[26] == palavra[36] {
            if palavra[27] == palavra[35] {
                if palavra[28] == palavra[34] {
                    if palavra[29] == palavra[33] {
                        if palavra[30] == palavra[32] {
                            return 13 * 2;
                        }
                    }
                }
            }
        } else if palavra[28] == palavra[36] {
            if palavra[29] == palavra[35] {
                if palavra[30] == palavra[34] {
                    if palavra[31] == palavra[33] {
                        return 14 * 2;
                    }
                }
            }
        } else if palavra[30] == palavra[36] {
            if palavra[31] == palavra[35] {
                if palavra[32] == palavra[34] {
                    return 15 * 2;
                }
            }
        } else if palavra[32] == palavra[36] {
            if palavra[33] == palavra[35] {
                return 16 * 2;
            }
        } else if palavra[34] == palavra[36] {
            return 17 * 2;
        }
    }

    return 18;
}
