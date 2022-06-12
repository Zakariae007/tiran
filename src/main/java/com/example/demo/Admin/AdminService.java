package com.example.demo.Admin;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.stereotype.Service;

import java.util.List;
import java.util.Optional;

@Service
public class AdminService {

    private final AdminRepository adminRepository;

    @Autowired
    public AdminService(AdminRepository adminRepository) {
        this.adminRepository = adminRepository;
    }

    public List<Admin> getAdmin(){
        return adminRepository.findAll();
    }

    public void addAdmin(Admin admin) {
        Optional<Admin> adminByEmail = Optional.ofNullable(adminRepository
                .findAdminByEmail(admin.getEmail()));

        if (adminByEmail.isPresent()){
            throw new IllegalStateException("Email taken. Please use a new email");
        }

        adminRepository.save(admin);
    }

    public String adminByID(String Id) {
        Optional<Admin> adminFound = Optional.ofNullable(adminRepository.findAdminById(Id));
        if (adminFound.isEmpty()){
            throw new IllegalStateException("Could not find an Admin with this ID");
        } else {
            // Return the actual admin in this case
            return "User was found";
        }
    }
}
