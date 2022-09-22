
fn check_palindrome(palavra: &Vec<u8>,index: usize) -> u64 {
    if palavra[index] == palavra[20+index] {
        if palavra[1+index] == palavra[19+index] {
            if palavra[2+index] == palavra[18+index] {
                if palavra[3+index] == palavra[17+index] {
                    if palavra[4+index] == palavra[16+index] {
                        if palavra[5+index] == palavra[15+index] {
                            if palavra[6+index] == palavra[14+index] {
                                if palavra[7+index] == palavra[13+index] {
                                    if palavra[8+index] == palavra[12+index] {
                                        if palavra[9+index] == palavra[11+index] {
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
    } else if palavra[2+index] == palavra[20+index] {
        if palavra[3+index] == palavra[19+index] {
            if palavra[4+index] == palavra[18+index] {
                if palavra[5+index] == palavra[17+index] {
                    if palavra[6+index] == palavra[16+index] {
                        if palavra[7+index] == palavra[15+index] {
                            if palavra[8+index] == palavra[14+index] {
                                if palavra[9+index] == palavra[13+index] {
                                    if palavra[10+index] == palavra[12+index] {
                                        return 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if palavra[4+index] == palavra[20+index] {
        if palavra[5+index] == palavra[19+index] {
            if palavra[6+index] == palavra[18+index] {
                if palavra[7+index] == palavra[17+index] {
                    if palavra[8+index] == palavra[16+index] {
                        if palavra[9+index] == palavra[15+index] {
                            if palavra[10+index] == palavra[14+index] {
                                if palavra[11+index] == palavra[13+index] {
                                    return 2;
                                }
                            }
                        }
                    }
                }
            }
        }
    } else if palavra[6+index] == palavra[20+index] {
        if palavra[7+index] == palavra[19+index] {
            if palavra[8+index] == palavra[18+index] {
                if palavra[9+index] == palavra[17+index] {
                    if palavra[10+index] == palavra[16+index] {
                        if palavra[11+index] == palavra[15+index] {
                            if palavra[12+index] == palavra[14+index] {
                                return 3;
                            }
                        }
                    }
                }
            }
        }
    } else if palavra[8+index] == palavra[20+index] {
        if palavra[9+index] == palavra[19+index] {
            if palavra[10+index] == palavra[18+index] {
                if palavra[11+index] == palavra[17+index] {
                    if palavra[12+index] == palavra[16+index] {
                        if palavra[13+index] == palavra[15+index] {
                            return 4;
                        }
                    }
                }
            }
        }
    } else if palavra[10+index] == palavra[20+index] {
        if palavra[11+index] == palavra[19+index] {
            if palavra[12+index] == palavra[18+index] {
                if palavra[13+index] == palavra[17+index] {
                    if palavra[14+index] == palavra[16+index] {
                        return 5;
                    }
                }
            }
        }
    } else if palavra[12+index] == palavra[20+index] {
        if palavra[13+index] == palavra[19+index] {
            if palavra[14+index] == palavra[18+index] {
                if palavra[15+index] == palavra[17+index] {
                    return 6;
                }
            }
        }
    } else if palavra[14+index] == palavra[20+index] {
        if palavra[15+index] == palavra[19+index] {
            if palavra[16+index] == palavra[18+index] {
                return 7;
            }
        }
    } else if palavra[16+index] == palavra[20+index] {
        if palavra[17+index] == palavra[19+index] {
            return 8;
        }
    } else if palavra[18+index] == palavra[20+index] {
        return 9;
    }

    return 10;
}
