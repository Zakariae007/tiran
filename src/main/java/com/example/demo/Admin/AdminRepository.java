package com.example.demo.Admin;

import org.springframework.data.mongodb.repository.Query;
import org.springframework.stereotype.Repository;
import org.springframework.data.mongodb.repository.MongoRepository;

import java.util.Optional;

@Repository
public interface AdminRepository
        extends MongoRepository<Admin, String> {

    @Override
    Optional<Admin> findById(String s);

    @Query("{email:'?0'}")
    Admin findAdminByEmail(String email);

    @Query("{id:'?0'}")
    Admin findAdminById(String Id);
}
