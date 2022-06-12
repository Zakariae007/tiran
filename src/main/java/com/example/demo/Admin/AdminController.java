package com.example.demo.Admin;

import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.*;

import java.util.List;

@RestController
@RequestMapping(path="api/admin")
public class AdminController {

    private final AdminService adminService;

    @Autowired
    public AdminController(AdminService adminService) {
        this.adminService = adminService;
    }


    //Add Admin
    @PostMapping("/addAdmin")
    public void registerAdmin(@RequestBody Admin admin){
        adminService.addAdmin(admin);
    }

    //Get Admin's Data
    @GetMapping("/admin/id")
    public String getAdminById(String Id){ return adminService.adminByID(Id);}

    //Get All admins
    @GetMapping("/allAdmins")
    public List<Admin> getAdmin(){
        return adminService.getAdmin();
    }

}
